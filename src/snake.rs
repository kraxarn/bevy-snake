use bevy::input::Input;
use bevy::prelude::{Color, Commands, Component, default, Entity, EventReader, EventWriter, KeyCode, Or, Query, Res, ResMut, Sprite, SpriteBundle, With};
use crate::{ARENA_HEIGHT, ARENA_WIDTH, Position, Size};
use crate::direction::Direction;
use crate::food::Food;
use crate::snake_segment::{SnakeSegment, SnakeSegments, spawn_segment};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct SnakeHead {
	direction: Direction,
}

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

pub struct EatEvent;

pub struct DieEvent;

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
	let head = commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: SNAKE_HEAD_COLOR,
				..default()
			},
			..default()
		})
		.insert(SnakeHead {
			direction: Direction::Up,
		})
		.insert(SnakeSegment)
		.insert(Position { x: 3, y: 3 })
		.insert(Size::new_square(0.8))
		.id();

	let tail = spawn_segment(commands, Position::new(3, 2));

	*segments = SnakeSegments(vec![head, tail]);
}

pub fn snake_movement_input(input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
	for mut head in &mut heads {
		let dir = if input.pressed(KeyCode::Left) {
			Direction::Left
		} else if input.pressed(KeyCode::Down) {
			Direction::Down
		} else if input.pressed(KeyCode::Up) {
			Direction::Up
		} else if input.pressed(KeyCode::Right) {
			Direction::Right
		} else {
			head.direction
		};

		if dir != head.direction.opposite() {
			head.direction = dir;
		}
	}
}

pub fn snake_movement(
	segments: ResMut<SnakeSegments>,
	mut last_tail_position: ResMut<LastTailPosition>,
	mut heads: Query<(Entity, &SnakeHead)>,
	mut positions: Query<&mut Position>,
	mut die_writer: EventWriter<DieEvent>,
) {
	// TODO: Why not use positions directly? Previous positions? Order?
	let segment_positions = segments
		.iter()
		.map(|e| *positions.get_mut(*e).unwrap())
		.collect::<Vec<Position>>();

	last_tail_position.0 = Some(*segment_positions.last().unwrap());

	for (entity, head) in &mut heads {
		let mut head_position = positions
			.get_mut(entity)
			.unwrap();

		match &head.direction {
			Direction::Left => {
				head_position.x -= 1;
			}
			Direction::Up => {
				head_position.y += 1;
			}
			Direction::Right => {
				head_position.x += 1;
			}
			Direction::Down => {
				head_position.y -= 1;
			}
		};

		if head_position.x < 0 || head_position.y < 0
			|| head_position.x as u32 >= ARENA_WIDTH
			|| head_position.y as u32 >= ARENA_HEIGHT
		{
			die_writer.send(DieEvent);
		}

		if segment_positions.contains(&head_position) {
			die_writer.send(DieEvent);
		}

		for (position, entity) in segment_positions
			.iter()
			.zip(segments.iter().skip(1))
		{
			*positions.get_mut(*entity).unwrap() = *position;
		}
	}
}

pub fn snake_eating(
	mut commands: Commands,
	mut eat_writer: EventWriter<EatEvent>,
	head_positions: Query<&Position, With<SnakeHead>>,
	food_positions: Query<(Entity, &Position), With<Food>>,
) {
	for head_position in &head_positions {
		for (entity, food_position) in &food_positions {
			if food_position == head_position {
				commands.entity(entity).despawn();
				eat_writer.send(EatEvent);
			}
		}
	}
}

pub fn snake_eat(
	commands: Commands,
	last_tail_position: Res<LastTailPosition>,
	mut segments: ResMut<SnakeSegments>,
	eat_reader: EventReader<EatEvent>,
) {
	if !eat_reader.is_empty() {
		segments.push(spawn_segment(commands, last_tail_position.0.unwrap()))
	}
}

pub fn snake_die(
	mut commands: Commands,
	die_reader: EventReader<DieEvent>,
	segments: ResMut<SnakeSegments>,
	entities: Query<Entity, Or<(With<Food>, With<SnakeSegment>)>>,
) {
	if die_reader.is_empty() {
		return;
	}

	for entity in &entities {
		commands.entity(entity).despawn();
	}

	spawn_snake(commands, segments)
}