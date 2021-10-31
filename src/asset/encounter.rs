use std::time::Duration;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(serde::Deserialize, TypeUuid, Clone)]
#[uuid = "bfd6af4d-d180-4aa7-88ec-f133edb435eb"]
pub struct EncounterConfig {
	pub name: String,
	pub difficulty: f64,
	pub enemies: Vec<EncounterMember>,
}

#[derive(serde::Deserialize, Clone)]
pub struct EncounterMember {
	pub name: String,
	pub starting_position: Vec2,
	pub time_offset: Duration,
	pub index: usize,
}
