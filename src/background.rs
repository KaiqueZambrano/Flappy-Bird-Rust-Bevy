use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("images/background.png");

    cmd.spawn(SpriteBundle {
        texture: background,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(2.8, 0.85, 1.0),
            ..default()
        },
        ..default()
    });
}
