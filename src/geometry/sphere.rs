use crate::rays::RaycastHit;
use crate::geometry::Geometrical;
use image::Rgba;

use crate::rays::Ray;
use crate::vectors::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    position: Vec3f,
    radius: f32,
    color: Rgba<u8>,
}

impl Sphere {
    pub fn new(n_position: Vec3f, n_radius: f32, n_color: Rgba<u8>) -> Sphere {
        assert!(n_radius >= 0.0);
        Sphere {
            position: n_position,
            radius: n_radius,
            color: n_color,
        }
    }
}

impl Geometrical for Sphere {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit> {
        // Vector to sphere center from ray origin.
        let l = self.position - ray.origin;
        // Ray length to the closest point to sphere center.
        let t = l.dot(&ray.direction);
        // Actual ray with given direction and length.
        let p = ray.origin + ray.direction * t;

        // Distance from sphere center to ray.
        let y = (self.position - p).magnitude();

        //If distance is less or equal than sphere radius -> ray intersects sphere.
        if y < self.radius {
            let x = (self.radius * self.radius - y * y).sqrt();
            // Point of first intersection.
            let t_1 = p - Vec3f::from_one(x);
            // Normal to the sphere surface in intersection point.
            let normal = (t_1 - self.position).normalized();
            Some(RaycastHit::new(t_1, normal, self.color))
        } else {
            None
        }
    }
}
