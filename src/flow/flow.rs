use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::prelude::*;

use crate::fixed_timestep::CoreFixedSet;

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        {
            let fixed_update = app.get_schedule_mut(CoreSchedule::FixedUpdate).unwrap();
            fixed_update
                .configure_set(
                    FlowSet::MechanicUpdate
                        .before(FlowSet::EntityMovement)
                        .after(CoreFixedSet::PreUpdate),
                )
                .configure_set(FlowSet::EntityUpdate.before(FlowSet::EntityMovement))
                .configure_set(FlowSet::EntityMovement.before(FlowSet::EntitySpawn))
                .configure_set(
                    FlowSet::EntitySpawn
                        .before(FlowSet::UiUpdate)
                        .before(CoreFixedSet::UpdateFlush),
                )
                .configure_set(
                    FlowSet::UiUpdate
                        .after(CoreFixedSet::UpdateFlush)
                        .before(CoreFixedSet::PostUpdate),
                );
        }
        app.configure_set(
            FlowSet::VisualUpdate
                .after(CoreSet::UpdateFlush)
                .before(CoreSet::PostUpdate),
        )
        .configure_set(
            FlowSet::Debug
                .after(CoreSet::PostUpdate)
                .before(CoreSet::PostUpdateFlush),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum FlowSet {
    MechanicUpdate,
    EntityUpdate,
    EntityMovement,
    EntitySpawn,
    UiUpdate,
    VisualUpdate,
    Debug,
}

#[derive(Copy, SystemSet)]
pub enum EventSet<T: Send + Sync + 'static> {
    Sender,
    #[system_set(ignore_field)]
    _Data(PhantomData<T>),
}

impl<T: Send + Sync + 'static> Clone for EventSet<T> {
    fn clone(&self) -> Self {
        Self::Sender
    }
}

impl<T: Send + Sync + 'static> Debug for EventSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sender => {
                f.write_str("Sender")?;
            }
            Self::_Data(..) => unreachable!(),
        }
        Ok(())
    }
}

impl<T: Send + Sync + 'static> Hash for EventSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Sender => {
                state.write_u32(0);
            }
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> PartialEq for EventSet<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Sender => match other {
                Self::Sender => true,
                Self::_Data(..) => unreachable!(),
            },
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> Eq for EventSet<T> {}
