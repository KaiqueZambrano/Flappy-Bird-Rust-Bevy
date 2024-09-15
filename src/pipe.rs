use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::textures::GameTextures;
use crate::timer::PipeSpawnTimer;
use crate::movement::Velocity;

pub struct PipePlugin;

#[derive(Bundle)]
pub struct PipeBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Pipe;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pipe);
    }
}

fn spawn_pipe(mut cmd: Commands,
              mut timer: ResMut<PipeSpawnTimer>,
              time: Res<Time>,
              texture: Res<GameTextures>) {
    if timer.0.tick(time.delta()).finished() {
        let pipe_y = rand::thread_rng().gen_range(-150.0 .. 150.0);
        let pipe_height = 300.0;
        let gap_size = 100.0;

        cmd.spawn((
            PipeBundle {
                rigid_body: RigidBody::KinematicPositionBased,
                collider: Collider::cuboid(25.0, 150.0),
                velocity: Velocity { x: 200., y: 0. },
                sprite: SpriteBundle {
                    texture: texture.pipe.clone(),
                    transform: Transform {
                        translation: Vec3::new(400.0, pipe_y + gap_size / 2.0 + pipe_height / 2.0, 0.0),
                        rotation: Quat::from_rotation_z(std::f32::consts::PI),
                        ..default()
                    },
                    ..default()
                },
            },
            Pipe,
            ActiveEvents::COLLISION_EVENTS,
        ));

        cmd.spawn((
            PipeBundle {
                rigid_body: RigidBody::KinematicPositionBased,
                collider: Collider::cuboid(25.0, 150.0),
                velocity: Velocity { x: 200., y: 0. },
                sprite: SpriteBundle {
                    texture: texture.pipe.clone(),
                    transform: Transform {
                        translation: Vec3::new(400.0, pipe_y - gap_size / 2.0 - pipe_height / 2.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
            },
            Pipe,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
