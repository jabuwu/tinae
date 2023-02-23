use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TimeToLiveSystem {
    Update,
}

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            time_to_live_update
                .in_set(TimeToLiveSystem::Update)
                .in_base_set(CoreSet::PostUpdate),
        );
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TimeToLive {
    pub alive_time: f32,
}

impl TimeToLive {
    pub fn new(duration: Duration) -> Self {
        Self {
            alive_time: duration.as_secs_f32(),
        }
    }
}

fn time_to_live_update(
    mut time_to_live_query: Query<(Entity, &mut TimeToLive)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (time_to_live_entity, mut time_to_live) in time_to_live_query.iter_mut() {
        if time_to_live.alive_time <= 0. {
            if let Some(entity_commands) = commands.get_entity(time_to_live_entity) {
                entity_commands.despawn_recursive();
            }
        }
        time_to_live.alive_time -= time.delta_seconds();
    }
}
