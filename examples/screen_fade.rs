use bevy::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .add_system(fade_out)
        .add_system(fade_out_complete)
        .run();
}

#[derive(Hash)]
pub struct ExampleFadeOut;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn fade_out(mut screen_fade: ResMut<ScreenFade>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        screen_fade.fade_out(ExampleFadeOut);
    }
}

fn fade_out_complete(
    mut screen_fade_out_events: EventReader<ScreenFadeOutEvent>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    for event in screen_fade_out_events.iter() {
        if event.in_context(ExampleFadeOut) {
            screen_fade.fade_in();
        }
    }
}
