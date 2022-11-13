use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Component, Query, Res, Transform, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use bevy::window::Windows;

pub const AREA_WIDTH: u32 = 10;
pub const AREA_HEIGHT: u32 = 10;

const AREA_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct AreaBackground;

pub fn spawn_area_background(mut commands: Commands) {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: AREA_COLOR,
				..default()
			},
			transform: Transform {
				scale: Vec3::new(0.0, 0.0, 0.0),
				..default()
			},
			..default()
		})
		.insert(AreaBackground);
}

pub fn resize_area_background(
	windows: Res<Windows>,
	mut area_backgrounds: Query<&mut Transform, With<AreaBackground>>
) {
	if let Some(window) = windows.get_primary() {
		let size = window.width().min(window.height());
		for mut area_background in &mut area_backgrounds {
			area_background.scale = Vec3::new(size, size, 0.0);
		}
	}
}