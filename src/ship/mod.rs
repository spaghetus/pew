use crate::{
	asset::{BulletPatternConfig, ShipConfig},
	bullet::BulletPattern,
	HEIGHT,
};
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
			.add_system(ship_pop_s.system())
			.add_system(ship_move_s.system())
			.add_system(screen_bounds_s.system())
			.add_system(ship_shoot_s.system());
	}
}

/// Creates a Ship
fn ship_setup_s(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	c.spawn_bundle(OrthographicCameraBundle::new_2d());
	c.spawn().insert(Ship);
}

/// Populates a ship with config values
fn ship_pop_s(
	mut c: Commands,
	mut ship: Query<(Entity), (With<Ship>, Added<Ship>)>,
	ship_config: Res<Handle<ShipConfig>>,
	ship_config_: Res<Assets<ShipConfig>>,
	patterns: Res<Vec<Handle<BulletPatternConfig>>>,
	patterns_: Res<Assets<BulletPatternConfig>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let ship_config = ship_config_.get(ship_config.clone()).unwrap();
	let ship_material = materials.add(ship_config.color.into());
	let patterns: Vec<&BulletPatternConfig> = patterns
		.iter()
		.map(|handle| patterns_.get(handle.clone()).unwrap())
		.collect();
	let pattern = patterns
		.iter()
		.filter(|v| v.name == ship_config.starting_pattern)
		.next()
		.unwrap();
	let pattern: BulletPattern = pattern.clone().clone().into();

	for ship in ship.iter() {
		c.entity(ship)
			.insert_bundle(SpriteBundle {
				transform: Transform::from_translation(Vec3::new(0.0, -(HEIGHT / 2.) + 50., 0.0)),
				sprite: Sprite::new(ship_config.size),
				material: ship_material.clone(),
				..Default::default()
			})
			.insert(ShipWeapon(pattern.clone(), Instant::now()));
	}
}
