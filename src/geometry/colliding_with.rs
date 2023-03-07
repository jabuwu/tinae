use bevy::prelude::*;

use super::{Aabb, Circle, TransformedShape};

pub trait CollidingWith<T> {
    fn colliding_with(&self, other: &T) -> bool;
}

impl CollidingWith<Circle> for Circle {
    fn colliding_with(&self, other: &Circle) -> bool {
        self.position.distance(other.position) < (self.radius + other.radius) * 0.5
    }
}

impl CollidingWith<Aabb> for Circle {
    fn colliding_with(&self, other: &Aabb) -> bool {
        let relative_center = self.position - other.position;
        let offset_from_corner = (relative_center).abs() - other.size * 0.5;
        offset_from_corner.x.max(offset_from_corner.y).min(0.)
            + (offset_from_corner.max(Vec2::ZERO)).length()
            - self.radius * 0.5
            < 0.
    }
}

impl CollidingWith<Circle> for Aabb {
    fn colliding_with(&self, other: &Circle) -> bool {
        other.colliding_with(self)
    }
}

impl CollidingWith<Aabb> for Aabb {
    fn colliding_with(&self, other: &Aabb) -> bool {
        self.position.x - self.size.x * 0.5 <= other.position.x + other.size.x * 0.5
            && self.position.x + self.size.x * 0.5 >= other.position.x - other.size.x * 0.5
            && self.position.y - self.size.y * 0.5 <= other.position.y + other.size.y * 0.5
            && self.position.y + self.size.y * 0.5 >= other.position.y - other.size.y * 0.5
    }
}

impl CollidingWith<Circle> for TransformedShape {
    fn colliding_with(&self, other: &Circle) -> bool {
        self.colliding_with(&TransformedShape::from(*other))
    }
}

impl CollidingWith<Aabb> for TransformedShape {
    fn colliding_with(&self, other: &Aabb) -> bool {
        self.colliding_with(&TransformedShape::from(*other))
    }
}

impl CollidingWith<TransformedShape> for TransformedShape {
    fn colliding_with(&self, other: &TransformedShape) -> bool {
        transformed_shape_to_shape!(
            self,
            a,
            transformed_shape_to_shape!(other, b, a.colliding_with(&b), return false),
            false
        )
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;

    use crate::geometry::prelude::*;

    #[test]
    fn colliding_circle_circle() {
        let a = Circle {
            position: Vec2::ZERO,
            radius: 1.,
        };
        let b = Circle {
            position: Vec2::splat(0.5),
            radius: 1.,
        };
        let c = Circle {
            position: Vec2::splat(1.5),
            radius: 1.,
        };
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn colliding_circle_aabb() {
        let a = Circle {
            position: Vec2::ZERO,
            radius: 1.,
        };
        let b = Aabb {
            position: Vec2::splat(0.2),
            size: Vec2::ONE,
        };
        let c = Aabb {
            position: Vec2::splat(1.5),
            size: Vec2::ONE,
        };
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn colliding_aabb_aabb() {
        let a = Aabb {
            position: Vec2::ZERO,
            size: Vec2::ONE,
        };
        let b = Aabb {
            position: Vec2::splat(0.5),
            size: Vec2::ONE,
        };
        let c = Aabb {
            position: Vec2::splat(1.5),
            size: Vec2::ONE,
        };
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn transformed_shape_colliding_none() {
        let a = Shape::None.at(Vec2::ZERO);
        let b = Shape::None.at(Vec2::ZERO);
        assert!(!a.colliding_with(&b));
    }

    #[test]
    fn transformed_shape_colliding_circle_circle() {
        let a = Shape::Circle { radius: 1. }.at(Vec2::ZERO);
        let b = Shape::Circle { radius: 1. }.at(Vec2::splat(0.5));
        let c = Shape::Circle { radius: 1. }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn transformed_shape_colliding_circle_aabb() {
        let a = Shape::Circle { radius: 1. }.at(Vec2::ZERO);
        let b = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(0.2));
        let c = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }

    #[test]
    fn transformed_shape_colliding_aabb_aabb() {
        let a = Shape::Aabb { size: Vec2::ONE }.at(Vec2::ZERO);
        let b = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(0.5));
        let c = Shape::Aabb { size: Vec2::ONE }.at(Vec2::splat(1.5));
        assert!(a.colliding_with(&b));
        assert!(!a.colliding_with(&c));
    }
}
