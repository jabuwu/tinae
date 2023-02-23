use std::time::Duration;

use bevy::prelude::*;

use crate::fixed_timestep::CoreFixedSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TimeToLiveSystem {
    Update,
}

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            time_to_live_update
                .in_set(TimeToLiveSystem::Update)
                .in_base_set(CoreFixedSet::PostUpdate),
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
    mut commands: Commands,
    time: Res<FixedTime>,
) {
    for (time_to_live_entity, mut time_to_live) in time_to_live_query.iter_mut() {
        if time_to_live.alive_time <= 0. {
            if let Some(entity_commands) = commands.get_entity(time_to_live_entity) {
                entity_commands.despawn_recursive();
            }
        }
        time_to_live.alive_time -= time.period.as_secs_f32();
    }
}
