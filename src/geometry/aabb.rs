use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub position: Vec2,
    pub size: Vec2,
}
