use bevy::reflect::TypeUuid;

#[derive(serde::Deserialize, TypeUuid, Clone)]
#[uuid = "f278a461-b662-4dc2-9606-bc2d3850861c"]
pub struct EnemyConfig {
	pub name: String,
	pub health: u32,
	pub weapon: String,
	/// This is the name of a bullet pattern; they are also used for enemy movement
	pub movement: String,
}
