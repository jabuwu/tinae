use bevy::prelude::*;

use super::ClearFlag;

pub trait AddFixedEvent {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self;
}

impl AddFixedEvent for App {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self {
        self.init_resource::<ClearFlag<Events<T>>>()
            .init_resource::<Events<T>>()
            .add_system_to_schedule(CoreSchedule::FixedUpdate, set_clear_fixed_events_flag::<T>)
            .add_system(clear_fixed_events::<T>.in_base_set(CoreSet::Last));
        self
    }
}

fn set_clear_fixed_events_flag<T: Event>(mut clear_fixed_events: ResMut<ClearFlag<Events<T>>>) {
    clear_fixed_events.clear = true;
}

fn clear_fixed_events<T: Event>(
    mut clear_fixed_events: ResMut<ClearFlag<Events<T>>>,
    mut events: ResMut<Events<T>>,
) {
    if clear_fixed_events.clear {
        events.update();
        clear_fixed_events.clear = false;
    }
}
