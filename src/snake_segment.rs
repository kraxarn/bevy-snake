use bevy::prelude::{Color, Commands, Component, Deref, DerefMut, Entity};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use crate::{Position, Size};

const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: SNAKE_SEGMENT_COLOR,
				..default()
			},
			..default()
		})
		.insert(SnakeSegment)
		.insert(position)
		.insert(Size::new_square(0.75))
		.id()
}