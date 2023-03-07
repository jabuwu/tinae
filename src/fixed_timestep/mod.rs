use std::marker::PhantomData;

use bevy::{
    prelude::*,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};

use crate::transform2::{update_transform2, Transform2System};

// TODO: yoinking transform systems into fixed update may require more thought...
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum FixedTransformSystem {
    UpdateTransform2,
    TransformPropagate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum CoreFixedSet {
    PreUpdate,
    Update,
    UpdateFlush,
    PostUpdate,
}

pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        {
            let schedule = app.get_schedule_mut(CoreSchedule::FixedUpdate).unwrap();
            schedule
                .set_default_base_set(CoreFixedSet::Update)
                .configure_set(CoreFixedSet::PreUpdate.before(CoreFixedSet::Update))
                .configure_set(CoreFixedSet::Update.before(CoreFixedSet::UpdateFlush))
                .configure_set(CoreFixedSet::UpdateFlush.before(CoreFixedSet::PostUpdate));
        }
        app.add_plugin(FixedInputPlugin)
            .insert_resource(FixedTime::new_from_secs(1. / 120.))
            .add_system(
                apply_system_buffers
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_base_set(CoreFixedSet::UpdateFlush),
            )
            .add_system(
                update_transform2
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::UpdateTransform2)
                    .in_base_set(CoreFixedSet::PostUpdate)
                    .before(FixedTransformSystem::TransformPropagate)
                    .after(Transform2System::TransformVisualPropagate),
            )
            .add_system(
                sync_simple_transforms
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::TransformPropagate)
                    .in_base_set(CoreFixedSet::PostUpdate),
            )
            .add_system(
                propagate_transforms
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::TransformPropagate)
                    .in_base_set(CoreFixedSet::PostUpdate),
            );
    }
}

#[derive(Resource)]
pub(crate) struct ClearFlag<T> {
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T> Default for ClearFlag<T> {
    fn default() -> Self {
        Self {
            clear: false,
            _marker: PhantomData,
        }
    }
}

mod events;
mod input;

pub use events::*;
pub use input::*;

pub mod prelude {
    pub use super::{AddFixedEvent, FixedInput, FixedInputSystem};
}
