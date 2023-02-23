use bevy::prelude::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, _app: &mut App) {}
}

mod scenes;

pub use scenes::*;

pub mod prelude {
    pub use super::AddScenes;
}
