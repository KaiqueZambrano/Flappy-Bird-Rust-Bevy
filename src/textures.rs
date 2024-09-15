use bevy::prelude::*;

pub struct GameTexturesPlugin;

#[derive(Resource)]
pub struct GameTextures {
    pub bird_up: Handle<Image>,
    pub bird_down: Handle<Image>,
    pub background: Handle<Image>,
    pub pipe: Handle<Image>,
}

impl Plugin for GameTexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_textures);
    }
}

fn load_textures(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let bird_up = asset_server.load("images/bird-up.png");
    let bird_down = asset_server.load("images/bird-down.png");
    let background = asset_server.load("images/background.png");
    let pipe = asset_server.load("images/pipe.png");

    cmd.insert_resource(GameTextures {
        bird_up: bird_up.clone(),
        bird_down: bird_down.clone(),
        background: background.clone(),
        pipe: pipe.clone(),
    });
}
