use std::time::{Duration, Instant};

use anyhow::Result;
use bevy::prelude::*;
use meval::{Context, Expr};

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 400.0;

fn main() {
	App::build()
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
		.run();
}

struct Ship;
struct Bullet(pub usize);
struct BulletInitialState(pub Vec3, pub Instant);
struct ShipWeapon(pub BulletPattern, pub Instant);
#[derive(Clone)]
struct BulletPattern(pub Expr, pub Expr, pub usize, pub Duration);
impl BulletPattern {
	fn eval(&self, start: Vec3, time: Duration) -> Result<Vec3> {
		Ok(start
			+ Vec3::new(
				self.0
					.eval_with_context(Context::default().var("t", time.as_secs_f64()))? as f32,
				self.1
					.eval_with_context(Context::default().var("t", time.as_secs_f64()))? as f32,
				0.0,
			))
	}
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
		BulletPattern(
			"0".parse().unwrap(),
			"t*300".parse().unwrap(),
			1,
			Duration::from_secs_f64(0.1),
		),
		Instant::now(),
	));
}

/// Moves Bullets according to their bullet pattern.
fn bullet_move_s(
	mut bullets: Query<(&mut Transform, &BulletPattern, &BulletInitialState), With<Bullet>>,
) {
	let now = Instant::now();
	for (mut t, b, s) in bullets.iter_mut() {
		t.translation = b.eval(s.0, now - s.1).unwrap_or(Vec3::ZERO);
	}
}

/// Inserts StartingPosition with the current position and time when Bullet is added.
fn bullet_add_s(bullets: Query<(Entity, &Transform), Added<Bullet>>, mut c: Commands) {
	for (e, t) in bullets.iter() {
		c.entity(e)
			.insert(BulletInitialState(t.translation, Instant::now()));
	}
}

const SHIP_SPEED: f32 = 300.0;

/// Moves the Ship according to the keyboard input.
fn ship_move_s(
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
fn ship_shoot_s(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Transform, &mut ShipWeapon), With<Ship>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut c: Commands,
) {
	for (t, mut sb) in query.iter_mut() {
		if keyboard_input.pressed(KeyCode::Space) {
			let now = Instant::now();
			if now - sb.1 > sb.0 .3 {
				let bullet_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
				for n in 0..sb.0 .2 {
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
fn screen_bounds_s(mut query: Query<&mut Transform, With<Ship>>) {
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

/// Despawn Bullets when they leave the screen.
fn bullet_despawn_s(query: Query<(&Transform, Entity), With<Bullet>>, mut c: Commands) {
	for (t, e) in query.iter() {
		if t.translation.x < -WIDTH / 2.0
			|| t.translation.x > WIDTH / 2.0
			|| t.translation.y < -HEIGHT / 2.0
			|| t.translation.y > HEIGHT / 2.0
		{
			c.entity(e).despawn();
		}
	}
}
