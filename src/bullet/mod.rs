use bevy::prelude::*;

mod bullet;
pub use bullet::*;

mod pattern;
pub use pattern::*;

mod collide;
pub use collide::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system(bullet_move_s.system())
			.add_system(bullet_despawn_s.system())
			.add_system(bullet_add_s.system());
	}
}
