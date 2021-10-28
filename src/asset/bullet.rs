use std::time::Duration;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use meval::Expr;

use crate::bullet::BulletPattern;

#[derive(serde::Deserialize, TypeUuid, Clone)]
#[uuid = "ec1fb650-3f30-470a-84d8-8b0f11a0f7dc"]
pub struct BulletPatternConfig {
	pub name: String,
	pub x: Expr,
	pub y: Expr,
	pub count: usize,
	pub delay: Duration,
}

impl Into<BulletPattern> for BulletPatternConfig {
	fn into(self) -> BulletPattern {
		BulletPattern {
			x: self.x,
			y: self.y,
			count: self.count,
			delay: self.delay,
			inner: None,
		}
	}
}
