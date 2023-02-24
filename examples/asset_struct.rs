use bevy::prelude::*;
use bevy_spine::prelude::*;
use tinae::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum ExampleSystem {
    LoadAssets,
    Setup,
}

fn main() {
    App::new()
        .init_resource::<AssetLibrary>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(load_assets.in_set(ExampleSystem::LoadAssets))
        .add_startup_system(
            setup
                .in_set(ExampleSystem::Setup)
                .after(ExampleSystem::LoadAssets),
        )
        .add_system(load_assets_status)
        .run();
}

#[derive(Default, Resource, AssetStruct)]
pub struct AssetLibrary {
    #[asset("FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[spine_asset("hammystar/skeleton.json", "hammystar/skeleton.atlas")]
    pub spine: Handle<SkeletonData>,
}

fn load_assets(world: &mut bevy::ecs::world::World) {
    world.resource_scope(|world, mut asset_library: Mut<AssetLibrary>| {
        asset_library.load(world);
    });
}

fn setup(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpineBundle {
        skeleton: asset_library.spine.clone(),
        ..Default::default()
    });
}

fn load_assets_status(world: &mut bevy::ecs::world::World) {
    world.resource_scope(|world, mut asset_library: Mut<AssetLibrary>| {
        dbg!(asset_library.status(world));
    });
}
