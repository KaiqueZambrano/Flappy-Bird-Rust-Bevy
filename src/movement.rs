use bevy::prelude::*;
use crate::bird::Bird;
use crate::pipe::Pipe;
use crate::textures::GameTextures;

pub struct MovementPlugin;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, user_input);
        app.add_systems(Update, apply_velocity);
        app.add_systems(Update, move_pipes);
    }
}

fn user_input(key: Res<ButtonInput<KeyCode>>,
              mut query: Query<(&mut Velocity, &mut Handle<Image>), With<Bird>>,
              game_textures: Res<GameTextures>) {
    for (mut velocity, mut texture_handle) in query.iter_mut() {
        if key.just_pressed(KeyCode::Space) {
            velocity.y = 200.0;
            *texture_handle = game_textures.bird_up.clone();
        }
        else {
            velocity.y -= 15.0;
            *texture_handle = game_textures.bird_down.clone(); 
        }
    }   
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform), With<Bird>>,
                  time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn move_pipes(mut cmd: Commands,
              mut query: Query<(Entity, &Velocity, &mut Transform), With<Pipe>>,
              time: Res<Time>) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation.x -= velocity.x * time.delta_seconds(); // Velocidade dos canos

        if transform.translation.x < -450.0 {
            cmd.entity(entity).despawn();
        }
    }
}
