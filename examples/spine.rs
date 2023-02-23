use bevy::prelude::*;
use bevy_spine::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .add_system(on_ready)
        .run();
}

#[derive(Component)]
pub struct HammyStar;

fn setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let skeleton = skeletons.add(SkeletonData::new_from_json(
        asset_server.load("hammystar/skeleton.json"),
        asset_server.load("hammystar/skeleton.atlas"),
    ));

    commands.spawn((
        SpineBundle {
            skeleton,
            ..Default::default()
        },
        HammyStar,
    ));
}

fn on_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<HammyStar>>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Some(mut spine) = spine_query.get_mut(spine_ready_event.entity).ok() {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", true);
        }
    }
}
