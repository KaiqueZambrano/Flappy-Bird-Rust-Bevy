use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::movement::Velocity;

pub struct BirdPlugin;

#[derive(Bundle)]
pub struct BirdBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Bird;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
        app.add_systems(Update, check_falling);
    }
}

fn spawn_bird(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let bird_down = asset_server.load("images/bird-down.png");

    cmd.spawn((
        BirdBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(15.0, 15.0),
            velocity: Velocity {x: 0., y: 0.},
            sprite: SpriteBundle {
                texture: bird_down,
                transform: Transform {
                    translation: Vec3::new(0.,200.,0.),
                    ..default()
                },
                ..default()
            },
        },
        Bird,
   ));
}

fn check_falling(
    query: Query<&Transform, With<Bird>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(transform) = query.get_single() {
        if transform.translation.y < -200.0 || transform.translation.y > 200.0 {
            exit.send(AppExit::Success);
        }
    }
}






