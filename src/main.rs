use bevy::app::CoreStage;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::utils::default;
use bevy::window::{WindowDescriptor, Windows};

use crate::area::{AREA_HEIGHT, AREA_WIDTH, resize_area_background, spawn_area_background};
use crate::food::spawn_food;
use crate::snake::{DieEvent, EatEvent, LastTailPosition, snake_die, snake_eat, snake_eating, snake_movement, snake_movement_input, spawn_snake};
use crate::snake_segment::SnakeSegments;

mod snake;
mod food;
mod direction;
mod snake_segment;
mod area;

const CLEAR_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: env!("CARGO_PKG_NAME").to_string(),
			width: 640.0,
			height: 640.0,
			..default()
		})
		.insert_resource(ClearColor(CLEAR_COLOR))
		.insert_resource(SnakeSegments::default()) // TODO: In snake?
		.insert_resource(LastTailPosition::default()) // TODO: In snake?
		.add_startup_system(spawn_area_background)
		.add_startup_system(setup_camera)
		.add_startup_system(spawn_snake)
		.add_system(resize_area_background)
		.add_system(snake_movement_input.before(snake_movement))
		.add_system_set_to_stage(CoreStage::PostUpdate,
			SystemSet::new()
				.with_system(position_translate)
				.with_system(size_scaling))
		.add_system_set(SystemSet::new()
			.with_run_criteria(FixedTimestep::step(0.15)) // TODO: Speed up after each collected
			.with_system(snake_movement)
			.with_system(snake_eating.after(snake_movement))
			.with_system(snake_die.after(snake_movement))
			.with_system(snake_eat.after(snake_eating))
			.with_system(spawn_food.after(snake_eat)))
		.add_event::<EatEvent>()
		.add_event::<DieEvent>()
		.add_plugins(DefaultPlugins)
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
	x: i32,
	y: i32,
}

impl Position {
	pub const fn new(x: i32, y: i32) -> Self {
		Self {
			x,
			y,
		}
	}
}

#[derive(Component)]
struct Size {
	width: f32,
	height: f32,
}

impl Size {
	pub const fn new_square(size: f32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>) {
	if let Some(window) = windows.get_primary() {
		for (sprite_size, mut transform) in &mut query {
			let x = sprite_size.width / AREA_WIDTH as f32 * window.width() as f32;
			let y = sprite_size.height / AREA_HEIGHT as f32 * window.height() as f32;
			let size = x.min(y);
			transform.scale = Vec3::new(size, size, 1.0);
		}
	}
}

fn position_translate(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
	fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
		let tile_size = bound_window / bound_game;
		pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
	}

	if let Some(window) = windows.get_primary() {
		for (pos, mut transform) in &mut query {
			let window_size = window.width().min(window.height());
			let x = convert(pos.x as f32, window_size, AREA_WIDTH as f32);
			let y = convert(pos.y as f32, window_size, AREA_HEIGHT as f32);
			transform.translation = Vec3::new(x, y, 1.0);
		}
	}
}