use bevy::prelude::{Color, Commands, Component};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use rand::random;
use crate::{ARENA_WIDTH, Position, Size};

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands) {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: FOOD_COLOR,
				..default()
			},
			..default()
		})
		.insert(Food)
		.insert(Position{
			// TODO: This spawns anywhere, not just on empty tiles
			x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
			y: (random::<f32>() * ARENA_WIDTH as f32) as i32,
		})
		.insert(Size::new_square(0.8));
}