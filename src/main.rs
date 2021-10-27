use std::time::{Duration, Instant};

use bevy::{prelude::*, tasks::TaskPool};

mod bullet;
use bullet::*;

mod ship;
use ship::*;

mod asset;
use asset::*;

mod stars;
use stars::*;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 400.0;

fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDescriptor {
			title: "Pew".to_string(),
			width: WIDTH,
			height: HEIGHT,
			vsync: true,
			resizable: false,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup_s.system())
		.add_system(bullet_move_s.system())
		.add_system(bullet_add_s.system())
		.add_system(ship_move_s.system())
		.add_system(screen_bounds_s.system())
		.add_system(ship_shoot_s.system())
		.add_system(bullet_despawn_s.system())
		.add_system(spawn_stars.system())
		.add_system(move_stars.system())
		.add_system(delete_stars.system())
		.run();
}

/// Creates a Ship and an EnemySpawner
fn setup_s(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
