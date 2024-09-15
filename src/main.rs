mod background;
mod bird;
mod camera;
mod textures;
mod timer;
mod pipe;
mod movement;
mod collision;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use background::BackgroundPlugin;
use bird::BirdPlugin;
use camera::CameraPlugin;
use textures::GameTexturesPlugin;
use timer::TimerPlugin;
use pipe::PipePlugin;
use movement::MovementPlugin;
use collision::CollisionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
			      primary_window: Some(Window {
				        title: "FLAPPY BIRD".into(),
				        resolution: (800., 400.).into(),
				        ..Default::default()
			      }),
			      ..Default::default()
		    }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(BackgroundPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GameTexturesPlugin)
        .add_plugins(TimerPlugin)
        .add_plugins(PipePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}


