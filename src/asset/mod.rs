use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

mod ship;
pub use ship::*;

mod bullet;
pub use bullet::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_plugin(RonAssetPlugin::<ShipConfig>::new(&["ship"]))
			.add_plugin(RonAssetPlugin::<BulletPatternConfig>::new(&["pattern"]))
			.add_startup_system(setup_assets_s.system());

		let world = app.world_mut();
		let server = world.get_resource::<AssetServer>().unwrap();
		let ship_config: Handle<ShipConfig> = server.load("conf.ship");
		let bullet_patterns: Vec<Handle<BulletPatternConfig>> = server
			.load_folder("bullet-patterns")
			.unwrap()
			.iter()
			.map(|handle| handle.clone().typed())
			.collect();
		app.insert_resource(ship_config);
		app.insert_resource(bullet_patterns);
	}
}

pub fn setup_assets_s(server: Res<AssetServer>, mut c: Commands) {
	server.watch_for_changes().unwrap();
}
