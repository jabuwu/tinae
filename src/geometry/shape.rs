use bevy::prelude::*;

use crate::transform2::Transform2;

use super::{Aabb, Circle, CollidingWith};

#[derive(Default, Copy, Clone)]
pub enum Shape {
    #[default]
    None,
    Circle {
        radius: f32,
    },
    Aabb {
        size: Vec2,
    },
}

impl Shape {
    pub fn at(&self, translation: Vec2) -> TransformedShape {
        TransformedShape {
            transform: Transform2::from_translation(translation),
            shape: *self,
        }
    }
}

pub struct TransformedShape {
    pub transform: Transform2,
    pub shape: Shape,
}

impl TransformedShape {
    pub fn colliding_with(&self, other: &TransformedShape) -> bool {
        macro_rules! match_shape {
            ($shape:expr, $transform:expr, $name:ident, $expr:expr) => {
                match $shape {
                    Shape::None => return false,
                    Shape::Circle { radius } => {
                        let $name = Circle {
                            position: $transform.translation,
                            radius: *radius,
                        };
                        $expr
                    }
                    Shape::Aabb { size } => {
                        let $name = Aabb {
                            position: $transform.translation,
                            size: *size,
                        };
                        $expr
                    }
                }
            };
        }
        match_shape!(
            &self.shape,
            self.transform,
            a,
            match_shape!(&other.shape, other.transform, b, a.colliding_with(&b))
        )
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;

    use crate::geometry::prelude::*;

    #[test]
    fn shape_colliding_none() {
        let a = Shape::None.at(Vec2::ZERO);
        let b = Shape::None.at(Vec2::ZERO);
        assert!(!a.colliding_with(&b));
    }

    #[test]
    fn shape_colliding_circle_circle() {
        let a = Shape::Circle { radius: 1. }.at(Vec2::ZERO);
        let b = Shape::Circle { radius: 1. }.at(Vec2::splat(0.5));
        let c = Shape::Circle { radius: 1. }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn shape_colliding_circle_aabb() {
        let a = Shape::Circle { radius: 1. }.at(Vec2::ZERO);
        let b = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(0.2));
        let c = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn shape_colliding_aabb_aabb() {
        let a = Shape::Aabb { size: Vec2::ONE }.at(Vec2::ZERO);
        let b = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(0.5));
        let c = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }
}
