use bevy::prelude::*;

pub struct TimerPlugin;

#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_timer);
    }
}

fn set_timer(mut cmd: Commands) {
    cmd.insert_resource(PipeSpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
}
