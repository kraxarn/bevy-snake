use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Component, Query, Res, Transform, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::sprite::Anchor::TopLeft;
use bevy::utils::default;
use bevy::window::{Window, Windows};

pub const AREA_WIDTH: u32 = 10;
pub const AREA_HEIGHT: u32 = 10;

const AREA_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct AreaBackground;

fn get_offset(window: &Window) -> Vec3 {
	let offset_x = window.width() / 2_f32;
	let offset_y = window.height() / 2_f32;

	Vec3::new(-offset_x, offset_y, 0.0)
}

pub fn spawn_area_background(
	windows: Res<Windows>,
	mut commands: Commands,
) {
	if let Some(window) = windows.get_primary() {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: AREA_COLOR,
					anchor: TopLeft,
					..default()
				},
				transform: Transform {
					scale: Vec3::new(64.0, 64.0, 1.0),
					translation: get_offset(window),
					..default()
				},
				..default()
			})
			.insert(AreaBackground);
	}
}

pub fn resize_area_background(
	windows: Res<Windows>,
	mut area_backgrounds: Query<&mut Transform, With<AreaBackground>>
) {
	if let Some(window) = windows.get_primary() {
		for mut area_background in &mut area_backgrounds {
			let size = window.width().min(window.height());
			area_background.scale = Vec3::new(size, size, 0.0);
			area_background.translation = get_offset(window);
		}
	}
}