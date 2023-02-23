use bevy::prelude::*;
use bevy::transform::TransformSystem;
use lerp::Lerp;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Transform2System {
    TransformPropagate,
}

pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_transform2
                .in_set(Transform2System::TransformPropagate)
                .in_base_set(CoreSet::PostUpdate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Transform2 {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform2 {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn with_rotation(self, rotation: f32) -> Self {
        Self { rotation, ..self }
    }

    pub fn with_scale(self, scale: Vec2) -> Self {
        Self { scale, ..self }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Depth {
    Inherit(f32),
    Exact(f32),
}

impl Default for Depth {
    fn default() -> Self {
        Self::Inherit(0.)
    }
}

impl Depth {
    pub fn depth_f32(&self) -> f32 {
        match *self {
            Depth::Inherit(depth) => 0.0_f32.lerp(0.01, depth),
            Depth::Exact(depth) => depth,
        }
    }
}

fn update_transform2(
    root_query: Query<Entity, Without<Parent>>,
    children_query: Query<&Children>,
    mut transform_query: Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
) {
    for root in root_query.iter() {
        update_transform2_recursive(root, &children_query, &mut transform_query, 0.);
    }
}

fn update_transform2_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    transform_query: &mut Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
    mut cumulative_depth: f32,
) {
    if let Some((mut transform, transform2, depth_layer)) = transform_query.get_mut(entity).ok() {
        if let Some(transform2) = transform2 {
            transform.translation.x = transform2.translation.x;
            transform.translation.y = transform2.translation.y;
            transform.scale = Vec3::new(transform2.scale.x, transform2.scale.y, 1.0);
            transform.rotation = Quat::from_rotation_z(transform2.rotation);
        }
        if let Some(depth_layer) = depth_layer {
            if matches!(depth_layer, Depth::Inherit(..)) {
                transform.translation.z = depth_layer.depth_f32();
            } else {
                transform.translation.z = depth_layer.depth_f32() - cumulative_depth;
            }
        }
        cumulative_depth += transform.translation.z;
    }
    if let Some(children) = children_query.get(entity).ok() {
        for child in children.iter() {
            update_transform2_recursive(*child, children_query, transform_query, cumulative_depth);
        }
    }
}
