use bevy::prelude::*;

pub struct AssetStructPlugin;

impl Plugin for AssetStructPlugin {
    fn build(&self, _app: &mut App) {}
}

mod asset_struct;
pub use asset_struct::*;

pub mod prelude {
    pub use super::{AssetStruct, AssetStructStatus};
}
