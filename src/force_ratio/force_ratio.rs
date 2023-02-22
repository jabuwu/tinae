use bevy::{prelude::*, transform::TransformSystem};

use crate::Persistent;

const RATIO_BAR_SIZE: f32 = 100000.;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ForceRatioSystem {
    Setup,
    Update,
}

pub struct ForceRatioPlugin;

impl Plugin for ForceRatioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ForceRatio>()
            .add_startup_system(force_ratio_setup.label(ForceRatioSystem::Setup))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                force_ratio_update
                    .label(ForceRatioSystem::Update)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Default, Resource, Copy, Clone, PartialEq)]
pub enum ForceRatio {
    #[default]
    Disabled,
    Enabled {
        width: f32,
        height: f32,
    },
}

#[derive(Component, Clone, Copy)]
pub enum ForceRatioBar {
    Top,
    Bottom,
    Left,
    Right,
}

impl ForceRatioBar {
    fn visibility(&self, force_ratio: &ForceRatio) -> Visibility {
        if matches!(force_ratio, &ForceRatio::Disabled) {
            Visibility::INVISIBLE
        } else {
            Visibility::VISIBLE
        }
    }
    fn translation(&self, force_ratio: &ForceRatio) -> Vec3 {
        match force_ratio {
            ForceRatio::Disabled => Vec3::ZERO,
            ForceRatio::Enabled { width, height } => match *self {
                ForceRatioBar::Top => Vec3::new(0., height * 0.5 + RATIO_BAR_SIZE * 0.5, 1.),
                ForceRatioBar::Bottom => Vec3::new(0., height * -0.5 - RATIO_BAR_SIZE * 0.5, 1.),
                ForceRatioBar::Left => Vec3::new(width * -0.5 - RATIO_BAR_SIZE * 0.5, 0., 1.),
                ForceRatioBar::Right => Vec3::new(width * 0.5 + RATIO_BAR_SIZE * 0.5, 0., 1.),
            },
        }
    }
}

fn force_ratio_setup(mut commands: Commands, force_ratio: Res<ForceRatio>) {
    for side in [
        ForceRatioBar::Top,
        ForceRatioBar::Bottom,
        ForceRatioBar::Left,
        ForceRatioBar::Right,
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
                    color: Color::BLACK,
                    ..Default::default()
                },
                visibility: side.visibility(force_ratio.as_ref()),
                transform: Transform::from_translation(side.translation(force_ratio.as_ref())),
                ..Default::default()
            },
            Persistent,
            side,
        ));
    }
}

fn force_ratio_update(
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    camera_query: Query<Entity, With<Camera>>,
    bar_query: Query<(Entity, &ForceRatioBar)>,
    windows: Res<Windows>,
    force_ratio: Res<ForceRatio>,
) {
    if let ForceRatio::Enabled { width, height } = force_ratio.as_ref() {
        if let Some(window) = windows.get_primary() {
            for camera_entity in camera_query.iter() {
                if let Some(mut camera_transform) = transform_query.get_mut(camera_entity).ok() {
                    let ratio = window.width() / window.height();
                    let mut desired_width = *width;
                    let mut desired_height = *height;
                    let desired_ratio = desired_width / desired_height;
                    if ratio > desired_ratio {
                        desired_width *= ratio / desired_ratio;
                    } else {
                        desired_height *= desired_ratio / ratio;
                    }
                    camera_transform.scale.x = desired_width / window.width();
                    camera_transform.scale.y = desired_height / window.height();
                }
            }
        }
    }
    for (bar_entity, bar) in bar_query.iter() {
        if let Some(mut bar_transform) = transform_query.get_mut(bar_entity).ok() {
            bar_transform.translation = bar.translation(force_ratio.as_ref());
        }
        if let Some(mut bar_visibility) = visibility_query.get_mut(bar_entity).ok() {
            *bar_visibility = bar.visibility(force_ratio.as_ref());
        }
    }
}
