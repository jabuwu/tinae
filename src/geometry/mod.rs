use bevy::prelude::*;

pub struct GeometryPlugin;

impl Plugin for GeometryPlugin {
    fn build(&self, _app: &mut App) {}
}

#[macro_use]
mod shape;

mod aabb;
mod circle;
mod colliding_with;
mod contains_point;

pub use crate::geometry::shape::*;
pub use aabb::*;
pub use circle::*;
pub use colliding_with::*;
pub use contains_point::*;

pub mod prelude {
    pub use super::{Aabb, Circle, CollidingWith, ContainsPoint, Shape};
}
