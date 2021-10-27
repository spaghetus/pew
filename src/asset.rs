use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::bullet::BulletPattern;

/// A colored rectangle, where the upper left and bottom right corners
/// are specified relative to the center of the character.
#[derive(serde::Deserialize)]
pub struct ColorRect(Color, (i32, i32, i32, i32));

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "f05a2760-6fd9-4b11-8933-2e0448d3560f"]
pub struct AssetEnemyGenerator {
	/// The name of the enemy.
	pub name: String,
	/// The name of the movement pattern the enemy uses.
	pub movement: String,
	/// The name of the shape of the enemy.
	pub shape: String,
	/// The name of the bullet pattern the enemy uses.
	pub bullet_pattern: String,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "055c1301-638b-4954-9747-de39ef567263"]
pub struct AssetBulletPattern {
	pub name: String,
	pub pattern: BulletPattern,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "055c1301-638b-4954-9747-de39ef567263"]
pub struct AssetShape {
	pub name: String,
	pub rects: Vec<ColorRect>,
}
