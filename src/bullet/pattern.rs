use anyhow::Result;
use bevy::prelude::*;
use meval::{Context, Expr};
use std::time::Duration;

#[derive(Clone, serde::Deserialize)]
pub struct BulletPattern {
	pub x: Expr,
	pub y: Expr,
	pub count: usize,
	pub delay: Duration,
	pub inner: Option<Box<BulletPattern>>,
}
impl BulletPattern {
	pub fn eval(&self, start: Vec3, time: Duration, index: usize) -> Result<Vec3> {
		Ok(start
			+ Vec3::new(
				self.x.eval_with_context(
					Context::default()
						.var("t", time.as_secs_f64())
						.var("n", (index % self.count) as f64)
						.var(
							"s",
							self.inner
								.as_ref()
								.map(|v| v.eval(start, time, index % self.count).unwrap().x)
								.unwrap_or(1.0) as f64,
						),
				)? as f32,
				self.y.eval_with_context(
					Context::default()
						.var("t", time.as_secs_f64())
						.var("n", (index % self.count) as f64)
						.var(
							"s",
							self.inner
								.as_ref()
								.map(|v| v.eval(start, time, index % self.count).unwrap().y)
								.unwrap_or(1.0) as f64,
						),
				)? as f32,
				0.0,
			))
	}
}
