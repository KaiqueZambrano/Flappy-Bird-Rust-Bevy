use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_rapier2d::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions);
    }
}

fn check_collisions(mut collision_events: EventReader<CollisionEvent>,
                    mut exit: EventWriter<AppExit>) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(..) => {
                exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}
