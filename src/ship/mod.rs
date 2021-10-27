use crate::{bullet::BulletPattern, HEIGHT};
use bevy::prelude::*;
use std::time::{Duration, Instant};

mod flight;
pub use flight::*;

mod weapon;
pub use weapon::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(ship_setup_s.system())
			.add_system(ship_move_s.system())
			.add_system(screen_bounds_s.system())
			.add_system(ship_shoot_s.system());
	}
}

/// Creates a Ship
fn ship_setup_s(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	c.spawn_bundle(OrthographicCameraBundle::new_2d());
	let ship_material = materials.add(Color::rgb(1.0, 0.8, 1.0).into());
	c.spawn_bundle(SpriteBundle {
		sprite: Sprite::new(Vec2::new(10.0, 16.0)),
		transform: Transform::from_translation(Vec3::new(0.0, -(HEIGHT / 2.) + 50., 0.0)),
		material: ship_material.clone(),
		..Default::default()
	})
	.insert(Ship)
	.insert(ShipWeapon(
		BulletPattern {
			x: "(sin(10*t*(n-2.5)/2)*50) + ((n-2.5)*5*t)".parse().unwrap(),
			y: "(t*150) + (cos(20*t)*30) - 30".parse().unwrap(),
			count: 5,
			delay: Duration::from_secs_f64(0.1),
			inner: None,
		},
		Instant::now(),
	));
}
