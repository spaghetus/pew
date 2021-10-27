use std::time::{Duration, Instant};

use bevy::prelude::*;

mod bullet;
use bullet::*;

mod ship;
use ship::*;

mod asset;
use asset::*;

mod stars;
use stars::*;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 400.0;

fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDescriptor {
			title: "Pew".to_string(),
			width: WIDTH,
			height: HEIGHT,
			vsync: true,
			resizable: false,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(BulletPlugin)
		.add_plugin(ShipPlugin)
		.add_plugin(StarsPlugin)
		.run();
}
