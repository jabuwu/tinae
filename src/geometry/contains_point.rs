use bevy::prelude::*;

use super::{Aabb, Circle, TransformedShape};

pub trait ContainsPoint {
    fn contains_point(&self, point: Vec2) -> bool;
}

impl ContainsPoint for Circle {
    fn contains_point(&self, point: Vec2) -> bool {
        self.position.distance(point) < self.radius * 0.5
    }
}

impl ContainsPoint for Aabb {
    fn contains_point(&self, point: Vec2) -> bool {
        point.x > self.position.x - self.size.x * 0.5
            && point.x < self.position.x + self.size.x * 0.5
            && point.y > self.position.y - self.size.y * 0.5
            && point.y < self.position.y + self.size.y * 0.5
    }
}

impl ContainsPoint for TransformedShape {
    fn contains_point(&self, point: Vec2) -> bool {
        transformed_shape_to_shape!(self, shape, shape.contains_point(point), false)
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;

    use crate::geometry::prelude::*;

    #[test]
    fn contains_point_circle() {
        let a = Circle {
            position: Vec2::ZERO,
            radius: 1.,
        };
        assert!(a.contains_point(Vec2::splat(0.25)));
        assert!(!a.contains_point(Vec2::splat(0.75)));
    }

    #[test]
    fn contains_point_aabb() {
        let a = Aabb {
            position: Vec2::ZERO,
            size: Vec2::ONE,
        };
        assert!(a.contains_point(Vec2::splat(0.25)));
        assert!(!a.contains_point(Vec2::splat(0.75)));
    }

    #[test]
    fn transformed_shape_contains_point_circle() {
        let a = Shape::Aabb { size: Vec2::ONE }.at(Vec2::ZERO);
        assert!(a.contains_point(Vec2::splat(0.25)));
        assert!(!a.contains_point(Vec2::splat(0.75)));
    }
}
