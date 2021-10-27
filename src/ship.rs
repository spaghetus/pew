use crate::bullet::*;
use crate::{HEIGHT, WIDTH};
use bevy::prelude::*;
use std::time::Instant;

pub struct Ship;
pub struct ShipWeapon(pub BulletPattern, pub Instant);

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

/// Spawns a Bullet from the Ship when the spacebar is pressed.
pub fn ship_shoot_s(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Transform, &mut ShipWeapon), With<Ship>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut c: Commands,
) {
	for (t, mut sb) in query.iter_mut() {
		if keyboard_input.pressed(KeyCode::Space) {
			let now = Instant::now();
			if now - sb.1 > sb.0.delay {
				let bullet_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
				for n in 0..sb.0.count {
					c.spawn_bundle(SpriteBundle {
						sprite: Sprite::new(Vec2::new(4.0, 4.0)),
						transform: Transform::from_translation(t.translation),
						material: bullet_material.clone(),
						..Default::default()
					})
					.insert(Bullet(n))
					.insert(sb.0.clone());
				}
				sb.1 = now;
			}
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
