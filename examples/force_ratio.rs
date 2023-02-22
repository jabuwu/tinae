use bevy::prelude::*;
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup.before(ForceRatioSystem::Setup))
        .run();
}

fn setup(mut commands: Commands, mut force_ratio: ResMut<ForceRatio>) {
    *force_ratio = ForceRatio::Enabled {
        width: 1024.,
        height: 1024.,
    };
    commands.spawn(Camera2dBundle::default());
}
