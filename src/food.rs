use bevy::prelude::{Color, Commands, Component, EventReader, Or, Query, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use rand::random;

use crate::{Position, Size};
use crate::area::{AREA_HEIGHT, AREA_WIDTH};
use crate::snake::EatEvent;
use crate::snake_segment::SnakeSegment;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
pub struct Food;

fn random_position() -> Position {
	Position {
		x: (random::<f32>() * AREA_WIDTH as f32) as i32,
		y: (random::<f32>() * AREA_HEIGHT as f32) as i32,
	}
}

pub fn spawn_food(
	mut commands: Commands,
	eat_event: EventReader<EatEvent>,
	entities: Query<&Position, Or<(With<Food>, With<SnakeSegment>)>>,
	foods: Query<With<Food>>,
) {
	if eat_event.is_empty() && !foods.is_empty() {
		return;
	}

	let positions = entities.iter().collect::<Vec<&Position>>();
	let mut position = random_position();
	while positions.contains(&&position) {
		// TODO: Infinite loop when nowhere to spawn
		position = random_position();
	}

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: FOOD_COLOR,
				..default()
			},
			..default()
		})
		.insert(Food)
		.insert(position)
		.insert(Size::new_square(0.8));
}