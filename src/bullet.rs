use crate::{HEIGHT, WIDTH};
use anyhow::Result;
use bevy::{prelude::*, tasks::TaskPool};
use meval::{Context, Expr};
use std::time::{Duration, Instant};

// Components

pub struct Bullet(pub usize);
pub struct BulletInitialState(pub Vec3, pub Instant);
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

// Systems

/// Moves Bullets according to their bullet pattern.
pub fn bullet_move_s(
	mut bullets: Query<(&mut Transform, &BulletPattern, &BulletInitialState, &Bullet)>,
) {
	let now = Instant::now();
	bullets.par_for_each_mut(&TaskPool::new(), 2048, |(mut t, b, s, bu)| {
		t.translation = b.eval(s.0, now - s.1, bu.0).unwrap_or(Vec3::ZERO);
	})
}

/// Inserts StartingPosition with the current position and time when Bullet is added.
pub fn bullet_add_s(bullets: Query<(Entity, &Transform), Added<Bullet>>, mut c: Commands) {
	for (e, t) in bullets.iter() {
		c.entity(e)
			.insert(BulletInitialState(t.translation, Instant::now()));
	}
}

/// Despawn Bullets when they leave the screen.
pub fn bullet_despawn_s(query: Query<(&Transform, Entity), With<Bullet>>, mut c: Commands) {
	for (t, e) in query.iter() {
		if t.translation.x < -WIDTH
			|| t.translation.x > WIDTH
			|| t.translation.y < -HEIGHT
			|| t.translation.y > HEIGHT
		{
			c.entity(e).despawn();
		}
	}
}

struct BulletHitEvent(pub Entity);
