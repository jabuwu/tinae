use bevy::prelude::*;

use crate::Persistent;

pub trait AddScenes {
    fn add_scenes<T: States>(&mut self) -> &mut Self;
}

impl AddScenes for App {
    fn add_scenes<T: States>(&mut self) -> &mut Self {
        self.add_state::<T>();
        for scene in T::variants() {
            self.add_system(clear_nonpersistent_entities.in_schedule(OnExit(scene)));
        }
        self
    }
}

fn clear_nonpersistent_entities(
    mut commands: Commands,
    entity_query: Query<Entity, (Without<Parent>, Without<Persistent>, Without<Window>)>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
