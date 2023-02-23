use bevy::prelude::*;
use tinae::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum ExampleSystem {
    PlayerSpawn,
    PlayerUpdate,
    EnemySpawn,
    EnemyUpdate,
    EnemySpawns,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_fixed_event::<PlayerSpawnEvent>()
        .add_fixed_event::<EnemySpawnEvent>()
        .add_startup_system(setup.in_set(EventSet::<PlayerSpawnEvent>::Sender))
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            player_spawn
                .in_set(ExampleSystem::PlayerSpawn)
                .in_base_set(FlowSet::EntitySpawn)
                .before(EventSet::<PlayerSpawnEvent>::Sender),
        )
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            player_update
                .in_set(ExampleSystem::PlayerUpdate)
                .in_base_set(FlowSet::EntityUpdate),
        )
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            enemy_spawn
                .in_set(ExampleSystem::EnemySpawn)
                .in_base_set(FlowSet::EntitySpawn)
                .before(EventSet::<PlayerSpawnEvent>::Sender),
        )
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            enemy_update
                .in_set(ExampleSystem::EnemyUpdate)
                .in_base_set(FlowSet::EntityUpdate),
        )
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            enemy_spawns
                .in_set(ExampleSystem::EnemySpawns)
                .in_base_set(FlowSet::MechanicUpdate)
                .in_set(EventSet::<EnemySpawnEvent>::Sender),
        )
        .run();
}

fn setup(mut commands: Commands, mut player_spawn_events: EventWriter<PlayerSpawnEvent>) {
    commands.spawn(Camera2dBundle::default());

    player_spawn_events.send_default();
}

#[derive(Default)]
struct PlayerSpawnEvent;

#[derive(Component)]
struct Player;

fn player_spawn(mut player_spawn_events: EventReader<PlayerSpawnEvent>, mut commands: Commands) {
    for _ in player_spawn_events.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(32.)),
                    color: Color::GREEN,
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::new(),
            Depth::Exact(1.),
            Player,
        ));
    }
}

fn player_update(
    mut player_query: Query<&mut Transform2, With<Player>>,
    keys: Res<FixedInput<KeyCode>>,
    time: Res<FixedTime>,
) {
    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if keys.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if keys.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation +=
            movement.normalize_or_zero() * time.period.as_secs_f32() * 300.;
    }
}

#[derive(Default)]
struct EnemySpawnEvent;

#[derive(Component)]
struct Enemy;

fn enemy_spawn(mut enemy_spawn_events: EventReader<EnemySpawnEvent>, mut commands: Commands) {
    for _ in enemy_spawn_events.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(32.)),
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::new(),
            Depth::Exact(0.),
            Enemy,
        ));
    }
}

fn enemy_update(
    mut transform_query: Query<&mut Transform2>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    time: Res<FixedTime>,
) {
    let player_position = if let Some(player_entity) = player_query.get_single().ok() {
        if let Some(player_transform) = transform_query.get(player_entity).ok() {
            player_transform.translation
        } else {
            Vec2::ZERO
        }
    } else {
        Vec2::ZERO
    };
    for enemy_entity in enemy_query.iter() {
        if let Some(mut enemy_transform) = transform_query.get_mut(enemy_entity).ok() {
            let direction = (player_position - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * time.period.as_secs_f32() * 100.;
        }
    }
}

#[derive(Default)]
struct EnemySpawns {
    spawn_timer: f32,
}

fn enemy_spawns(
    mut local: Local<EnemySpawns>,
    mut enemy_spawn_events: EventWriter<EnemySpawnEvent>,
    time: Res<FixedTime>,
) {
    if local.spawn_timer > 1. {
        enemy_spawn_events.send_default();
        local.spawn_timer -= 1.;
    }
    local.spawn_timer += time.period.as_secs_f32();
}
