use bevy::prelude::*;
use rand::Rng;

use crate::{HEIGHT, WIDTH};

pub struct Star(pub f64);

/// Randomly spawn stars at the top of the screen
pub fn spawn_stars(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	let mut rng = rand::thread_rng();
	let chance = 0.2;
	let material = materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5).into());
	if rng.gen::<f64>() < chance {
		c.spawn_bundle(SpriteBundle {
			sprite: Sprite::new(Vec2::new(2.0, 2.0)),
			material,
			transform: Transform::from_translation(Vec3::new(
				rng.gen::<f32>() * WIDTH - WIDTH / 2.0,
				HEIGHT / 2.0,
				0.0,
			)),
			..Default::default()
		})
		.insert(Star(rng.gen()));
	}
}

/// Move stars down depending on their random values
pub fn move_stars(mut stars: Query<(&mut Transform, &Star)>, time: Res<Time>) {
	for (mut transform, star) in &mut stars.iter_mut() {
		transform.translation.y -= ((star.0 + 0.5) * time.delta_seconds_f64() * 100.0) as f32;
		transform.translation.z = (star.0 - 0.5) as f32;
	}
}

/// Delete stars once they pass the bottom of the screen
pub fn delete_stars(stars: Query<(Entity, &Transform), With<Star>>, mut commands: Commands) {
	for (entity, transform) in stars.iter() {
		if transform.translation.y < -HEIGHT / 2.0 {
			commands.entity(entity).despawn();
		}
	}
}
