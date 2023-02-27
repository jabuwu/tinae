use bevy::prelude::*;
use bevy_spine::prelude::*;
use tinae::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum ExampleSystem {
    Setup,
    OnLoad,
    Status,
    Unload,
}

fn main() {
    App::new()
        .add_asset_struct::<AssetLibrary>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup.in_set(ExampleSystem::Setup))
        .add_system(on_load.in_set(ExampleSystem::OnLoad))
        .add_system(status.in_set(ExampleSystem::Status))
        .add_system(unload.in_set(ExampleSystem::Unload))
        .run();
}

#[derive(Default, Resource, AssetStruct)]
pub struct AssetLibrary {
    #[asset("FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[spine_asset("hammystar/skeleton.json", "hammystar/skeleton.atlas")]
    pub spine: Handle<SkeletonData>,
}

#[derive(Component)]
pub struct LoadingBarBackground;

#[derive(Component)]
pub struct LoadingBar;

fn setup(mut commands: Commands, mut load_events: EventWriter<AssetStructLoadEvent<AssetLibrary>>) {
    commands.spawn(Camera2dBundle::default());
    load_events.send_default();

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    color: Color::BLACK,
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::new().with_scale(Vec2::new(200., 40.)),
            Depth::Exact(0.),
            LoadingBarBackground,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1., 1.)),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Transform2::new().with_scale(Vec2::new(0., 1.)),
                Depth::Inherit(0.1),
                LoadingBar,
            ));
        });
}

fn on_load(
    mut complete_events: EventReader<AssetStructLoadedEvent<AssetLibrary>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in complete_events.iter() {
        commands.spawn(SpineBundle {
            skeleton: asset_library.spine.clone(),
            ..Default::default()
        });
    }
}

fn status(
    mut loading_bar_query: Query<&mut Transform2, With<LoadingBar>>,
    mut commands: Commands,
    loading_bar_background_query: Query<Entity, With<LoadingBarBackground>>,
    state: Res<AssetStructState<AssetLibrary>>,
) {
    match state.status() {
        AssetStructStatus::Loading { progress } => {
            for mut loading_bar_transform in loading_bar_query.iter_mut() {
                loading_bar_transform.scale.x = progress;
            }
        }
        AssetStructStatus::Loaded => {
            for loading_bar_background_entity in loading_bar_background_query.iter() {
                commands
                    .get_entity(loading_bar_background_entity)
                    .map(|entity| entity.despawn_recursive());
            }
        }
        _ => {}
    }
}

fn unload(
    mut commands: Commands,
    mut unload_events: EventWriter<AssetStructUnloadEvent<AssetLibrary>>,
    spine_query: Query<Entity, With<Spine>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        unload_events.send_default();
        for spine_entity in spine_query.iter() {
            commands
                .get_entity(spine_entity)
                .map(|entity| entity.despawn_recursive());
        }
    }
}
