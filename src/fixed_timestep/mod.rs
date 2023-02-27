use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum CoreFixedSet {
    PreUpdate,
    Update,
    PostUpdate,
}

pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        {
            let schedule = app.get_schedule_mut(CoreSchedule::FixedUpdate).unwrap();
            schedule.set_default_base_set(CoreFixedSet::Update);
        }
        app.add_plugin(FixedInputPlugin)
            .insert_resource(FixedTime::new_from_secs(1. / 120.));
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
