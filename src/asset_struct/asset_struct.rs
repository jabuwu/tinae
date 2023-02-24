use bevy::{ecs::system::Resource, prelude::*};
pub use tinae_macros::AssetStruct;

pub trait AssetStruct: Resource {
    fn load(&mut self, world: &mut World);
    fn status(&mut self, world: &mut World) -> AssetStructStatus;
}

#[derive(Default, Debug, Clone, Copy)]
pub enum AssetStructStatus {
    #[default]
    NotLoaded,
    Loading {
        progress: f32,
    },
    Loaded,
    Failed,
}
