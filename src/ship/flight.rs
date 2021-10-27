use bevy::prelude::*;

use crate::{HEIGHT, WIDTH};

pub struct Ship;

const SHIP_SPEED: f32 = 300.0;

/// Moves the Ship according to the keyboard input.
pub fn ship_move_s(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<&mut Transform, With<Ship>>,
	time: Res<Time>,
) {
	for mut t in query.iter_mut() {
		if keyboard_input.pressed(KeyCode::Left) {
			t.translation.x -= SHIP_SPEED * time.delta_seconds();
		}
		if keyboard_input.pressed(KeyCode::Right) {
			t.translation.x += SHIP_SPEED * time.delta_seconds();
		}
		if keyboard_input.pressed(KeyCode::Up) {
			t.translation.y += SHIP_SPEED * time.delta_seconds();
		}
		if keyboard_input.pressed(KeyCode::Down) {
			t.translation.y -= SHIP_SPEED * time.delta_seconds();
		}
	}
}

const COLLISION_BORDER: f32 = 20.0;

/// Prevent the Ship from moving out of the screen.
pub fn screen_bounds_s(mut query: Query<&mut Transform, With<Ship>>) {
	for mut t in query.iter_mut() {
		if t.translation.x < -(WIDTH / 2.0) + COLLISION_BORDER {
			t.translation.x = -(WIDTH / 2.0) + COLLISION_BORDER;
		}
		if t.translation.x > (WIDTH / 2.0) - COLLISION_BORDER {
			t.translation.x = (WIDTH / 2.0) - COLLISION_BORDER;
		}
		if t.translation.y < -(HEIGHT / 2.0) + COLLISION_BORDER {
			t.translation.y = -(HEIGHT / 2.0) + COLLISION_BORDER;
		}
		if t.translation.y > (HEIGHT / 2.0) - COLLISION_BORDER {
			t.translation.y = (HEIGHT / 2.0) - COLLISION_BORDER;
		}
	}
}
