use bevy::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .init_resource::<AssetLibrary>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Resource, AssetStruct)]
pub struct AssetLibrary {
    #[asset("FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
}

fn setup(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    asset_library.load_assets(asset_server.as_ref());
}
