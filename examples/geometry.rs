use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use tinae::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TinaePlugins)
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
pub struct Movement;

#[derive(Component)]
pub struct Collidable {
    shape: Shape,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::new(),
        Movement,
        Collidable {
            shape: Shape::Aabb {
                size: Vec2::splat(50.),
            },
        },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::from_xy(100., -75.),
        Collidable {
            shape: Shape::Aabb {
                size: Vec2::splat(50.),
            },
        },
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Transform2::from_xy(-150., 125.).with_scale(Vec2::splat(150.)),
        Collidable {
            shape: Shape::Circle { radius: 150. * 0.5 },
        },
    ));
}

fn movement(
    mut collidable_query: Query<(Entity, &mut Transform2, &Collidable)>,
    movement_query: Query<Entity, With<Movement>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
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

    for movement_entity in movement_query.iter() {
        let new_translation =
            if let Some((collidable_entity, collidable_transform, collidable_collidable)) =
                collidable_query.get(movement_entity).ok()
            {
                let new_translation = collidable_transform.translation
                    + movement.normalize_or_zero() * time.delta_seconds() * 300.;
                let new_transformed_shape = collidable_collidable.shape.at(new_translation);
                let mut can_move = true;
                for (other_collidable_entity, other_collidable_transform, other_collidable) in
                    collidable_query.iter()
                {
                    if other_collidable_entity != collidable_entity {
                        if new_transformed_shape.colliding_with(
                            &other_collidable
                                .shape
                                .at(other_collidable_transform.translation),
                        ) {
                            can_move = false;
                            break;
                        }
                    }
                }
                if can_move {
                    Some(new_translation)
                } else {
                    None
                }
            } else {
                None
            };
        if let Some(new_translation) = new_translation {
            if let Some((_, mut movement_transform, _)) =
                collidable_query.get_mut(movement_entity).ok()
            {
                movement_transform.translation = new_translation;
            }
        }
    }
}
