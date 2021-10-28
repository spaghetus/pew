use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "6e6af008-2719-4aa2-a08c-22ac29741549"]
pub struct ShipConfig {
	pub speed: f32,
	pub size: Vec2,
	pub color: Color,
	pub starting_pattern: String,
}
