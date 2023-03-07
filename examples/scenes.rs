use bevy::prelude::*;
use tinae::prelude::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppScene {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_scenes::<AppScene>()
        .add_startup_system(setup)
        .add_system(menu_setup.in_schedule(OnEnter(AppScene::Menu)))
        .add_system(
            menu_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(OnUpdate(AppScene::Menu)),
        )
        .add_system(game_setup.in_schedule(OnEnter(AppScene::Game)))
        .add_system(
            game_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(OnUpdate(AppScene::Game)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            TextSection::new(
                "Main Menu\n".to_owned(),
                TextStyle {
                    font: asset_server.load("./FiraSans-Bold.ttf"),
                    font_size: 86.,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "Press space to enter game".to_owned(),
                TextStyle {
                    font: asset_server.load("./FiraSans-Bold.ttf"),
                    font_size: 32.,
                    ..Default::default()
                },
            ),
        ])
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn menu_update(mut next_scene: ResMut<NextState<AppScene>>, keys: Res<FixedInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_scene.set(AppScene::Game);
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            TextSection::new(
                "Game\n".to_owned(),
                TextStyle {
                    font: asset_server.load("./FiraSans-Bold.ttf"),
                    font_size: 86.,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "Press escape to go back to menu".to_owned(),
                TextStyle {
                    font: asset_server.load("./FiraSans-Bold.ttf"),
                    font_size: 32.,
                    ..Default::default()
                },
            ),
        ])
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn game_update(mut next_scene: ResMut<NextState<AppScene>>, keys: Res<FixedInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_scene.set(AppScene::Menu);
    }
}
