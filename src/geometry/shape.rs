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

impl From<Circle> for TransformedShape {
    fn from(circle: Circle) -> Self {
        Self {
            shape: Shape::Circle {
                radius: circle.radius,
            },
            transform: Transform2::from_translation(circle.position),
        }
    }
}

impl From<Aabb> for TransformedShape {
    fn from(aabb: Aabb) -> Self {
        Self {
            shape: Shape::Aabb { size: aabb.size },
            transform: Transform2::from_translation(aabb.position),
        }
    }
}

macro_rules! transformed_shape_to_shape {
    ($transformed_shape:expr, $name:ident, $expr:expr, $none_expr:expr) => {
        match &$transformed_shape.shape {
            crate::geometry::Shape::None => $none_expr,
            crate::geometry::Shape::Circle { radius } => {
                let $name = crate::geometry::Circle {
                    position: $transformed_shape.transform.translation,
                    radius: *radius,
                };
                $expr
            }
            crate::geometry::Shape::Aabb { size } => {
                let $name = crate::geometry::Aabb {
                    position: $transformed_shape.transform.translation,
                    size: *size,
                };
                $expr
            }
        }
    };
}
