use std::time::Duration;

use bevy::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .add_system_to_schedule(CoreSchedule::FixedUpdate, spawn)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn(mut commands: Commands, mouse_buttons: Res<FixedInput<MouseButton>>, cursor: Res<Cursor>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::splat(32.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::from_translation(cursor.position),
            TimeToLive::new(Duration::from_millis(200)),
        ));
    }
}
