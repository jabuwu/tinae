use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Circle {
    pub position: Vec2,
    pub radius: f32,
}
