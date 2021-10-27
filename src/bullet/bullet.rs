use std::time::Instant;

use bevy::{prelude::*, tasks::TaskPool};

use crate::{HEIGHT, WIDTH};

use super::BulletPattern;

/// The index of the bullet among its weapon group and whether it was fired by the player.
pub struct Bullet(pub usize, pub bool);
pub struct BulletInitialState(pub Vec3, pub Instant);

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
