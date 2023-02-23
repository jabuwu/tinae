use std::hash::Hash;

use bevy::{input::InputSystem, prelude::*, reflect::Reflect};

use super::ClearFlag;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct FixedInputSystem;

pub trait AddFixedInput {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddFixedInput for App {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.init_resource::<ClearFlag<Input<T>>>()
            .init_resource::<FixedInput<T>>()
            .add_system(
                update_fixed_input::<T>
                    .in_base_set(CoreSet::PreUpdate)
                    .after(InputSystem),
            )
            .add_system_to_schedule(
                CoreSchedule::FixedUpdate,
                set_clear_fixed_input_flag::<T>.in_set(FixedInputSystem),
            )
            .add_system(clear_fixed_input::<T>.in_base_set(CoreSet::Last));
        self
    }
}

pub(crate) struct FixedInputPlugin;

impl Plugin for FixedInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_input::<KeyCode>();
        app.add_fixed_input::<ScanCode>();
        app.add_fixed_input::<MouseButton>();
        app.add_fixed_input::<GamepadButton>();
    }
}

#[derive(Debug, Clone, Resource, Reflect, Deref, DerefMut)]
#[reflect(Default)]
pub struct FixedInput<T: Copy + Eq + Hash + Send + Sync + 'static>(Input<T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for FixedInput<T> {
    fn default() -> Self {
        Self(Input::default())
    }
}

fn update_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    input: Res<Input<T>>,
) {
    for pressed in input.get_just_pressed() {
        fixed_input.press(*pressed);
    }
    for released in input.get_just_released() {
        fixed_input.release(*released);
    }
}

fn set_clear_fixed_input_flag<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut clear_fixed_input: ResMut<ClearFlag<Input<T>>>,
    mut fixed_input: ResMut<FixedInput<T>>,
) {
    if clear_fixed_input.clear {
        fixed_input.clear();
    }
    clear_fixed_input.clear = true;
}

fn clear_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    mut clear_fixed_input: ResMut<ClearFlag<Input<T>>>,
) {
    if clear_fixed_input.clear {
        fixed_input.clear();
    }
    clear_fixed_input.clear = false;
}
