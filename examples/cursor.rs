use bevy::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .add_system(follow_cursor)
        .run();
}

#[derive(Component)]
pub struct FollowCursor;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        FollowCursor,
    ));
}

fn follow_cursor(
    mut follow_cursor_query: Query<&mut Transform, With<FollowCursor>>,
    cursor: Res<Cursor>,
) {
    for mut follow_cursor_transform in follow_cursor_query.iter_mut() {
        follow_cursor_transform.translation = cursor.position.extend(0.);
    }
}
