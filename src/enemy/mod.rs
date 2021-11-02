use std::time::Instant;

use bevy::prelude::*;
use rand::{distributions::WeightedIndex, Rng};
use statrs::distribution::Continuous;

use crate::{
	asset::{BulletPatternConfig, EncounterConfig, EnemyConfig},
	bullet::BulletPattern,
	ship::ShipWeapon,
};

pub struct Encounter(pub Vec<Entity>);

pub struct EnemyTick(pub Timer);

pub struct Enemy {
	pub health: u32,
	pub movement: BulletPattern,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(EnemyTick(Timer::from_seconds(15.0, true)))
			.add_system(tick_enemy_tick.system())
			.add_system(despawn_encounter.system())
			.add_system(spawn_encounter.system());
	}
}

fn tick_enemy_tick(mut enemy_tick: ResMut<EnemyTick>, time: Res<Time>) {
	enemy_tick.0.tick(time.delta());
}

/// Despawn Encounter when all of its referenced entities are gone
fn despawn_encounter(
	enemy: Query<&Enemy>,
	encounter: Query<(Entity, &Encounter)>,
	mut c: Commands,
) {
	for (id, encounter) in encounter.iter() {
		let remaining = encounter
			.0
			.iter()
			.filter(|e| enemy.get(**e).is_ok())
			.count();
		if remaining == 0 {
			c.entity(id).despawn()
		}
	}
}

const SPAWN_STDEV: f64 = 1.0;

/// Spawn an Encounter when EnemyTick fires and Encounter doesn't exist.
fn spawn_encounter(
	tick: Res<EnemyTick>,
	mut c: Commands,
	enemies: Res<Vec<Handle<EnemyConfig>>>,
	enemies_: Res<Assets<EnemyConfig>>,
	encounters: Res<Vec<Handle<EncounterConfig>>>,
	encounters_: Res<Assets<EncounterConfig>>,
	patterns: Res<Vec<Handle<BulletPatternConfig>>>,
	patterns_: Res<Assets<BulletPatternConfig>>,
	time: Res<Time>,
) {
	// Coerce the EnemyConfigs and EncounterConfigs out of the assets
	let enemies: Vec<&EnemyConfig> = enemies
		.iter()
		.map(|handle| enemies_.get(handle.clone()).unwrap())
		.collect();
	let encounters: Vec<&EncounterConfig> = encounters
		.iter()
		.map(|handle| encounters_.get(handle.clone()).unwrap())
		.collect();
	let patterns: Vec<&BulletPatternConfig> = patterns
		.iter()
		.map(|handle| patterns_.get(handle.clone()).unwrap())
		.collect();
	// Build a normal distribution around the current time
	let now = time.seconds_since_startup() / 60.0;
	let norm = statrs::distribution::Normal::new(now, SPAWN_STDEV).unwrap();
	// Sample the normal distribution using the encounter's difficulty to build weigth
	let weights = encounters
		.iter()
		.map(|e| norm.pdf(e.difficulty))
		.collect::<Vec<f64>>();
	// Build a distribution from the weights
	let dist = WeightedIndex::new(weights).unwrap();
	// Sample the distribution
	let mut rng = rand::thread_rng();
	let index = rng.sample(dist);
	let encounter = encounters[index];
	// Spawn the enemies from the encounter and retain their IDs.
	let ids = encounter.enemies.iter().map(|enc| {
		let enemy = enemies
			.iter()
			.filter(|e| e.name == enc.name)
			.next()
			.unwrap();
		let movement: BulletPattern = patterns
			.iter()
			.filter(|p| p.name == enemy.movement)
			.next()
			.unwrap()
			.to_owned()
			.to_owned()
			.into();
		let weapon: BulletPattern = patterns
			.iter()
			.filter(|p| p.name == enemy.weapon)
			.next()
			.unwrap()
			.to_owned()
			.to_owned()
			.into();
		c.spawn()
			.insert(Enemy {
				health: enemy.health,
				movement,
			})
			.insert(ShipWeapon(weapon, Instant::now()))
			.insert_bundle(SpriteBundle {
				sprite: Sprite::new(Vec2::new(10.0, 10.0)),
				// TODO: add a way to configure an enemy's appearance
				..Default::default()
			})
			.id();
	});
}
