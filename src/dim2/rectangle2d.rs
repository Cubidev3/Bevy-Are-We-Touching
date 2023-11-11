use are_we_touching::dim2::aabb::{AABB2D, ToAABB2D};
use are_we_touching::dim2::convex::Convex2D;
use bevy::math::{Vec2, Quat, Vec3};
use bevy::prelude::Transform;

pub struct Rectangle2D {
    pub center: Vec2,
    pub extents: Vec2,
    pub rotation: Quat
}

impl Rectangle2D {
    pub fn new(center: Vec2, extents: Vec2, rotation: Quat) -> Rectangle2D {
        Rectangle2D { center, extents, rotation }
    }

    pub fn from_transform(transform: Transform, non_resized_extents: Vec2) -> Rectangle2D {
        Rectangle2D {
            center: transform.translation.truncate(),
            extents: transform.translation.truncate() * non_resized_extents, // Vec2 * Vec2 is Component-wise
            rotation: transform.rotation
        }
    }
}

impl ToAABB2D for Rectangle2D {
    fn to_aabb(&self) -> AABB2D {
        let rotated_extents = self.rotation.mul_vec3(Vec3::from((self.extents, 0))).truncate();
        AABB2D::new(self.center.into(), rotated_extents.into())
    }
}

impl Convex2D for Rectangle2D {
    fn support(&self, direction: mint::Vector2<f32>) -> Option<mint::Vector2<f32>> {
        let direction = Vec2::new(direction.x, direction.y);

    }
}