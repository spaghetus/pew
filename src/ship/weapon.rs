use super::Ship;
use crate::bullet::{Bullet, BulletPattern};
use bevy::prelude::*;
use std::time::Instant;

pub struct ShipWeapon(pub BulletPattern, pub Instant);

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
					.insert(Bullet(n, true))
					.insert(sb.0.clone());
				}
				sb.1 = now;
			}
		}
	}
}
