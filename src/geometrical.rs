extern crate image;

use crate::misc::RaycastHit;
use image::Rgba;

use crate::misc::Ray;
use crate::vectors::*;

pub trait Geometrical: std::fmt::Debug {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit>;
}

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
        if (y < self.radius) {
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
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    point_a: Vec3f,
    point_b: Vec3f,
    point_c: Vec3f,
    albedo: Rgba<u8>,
}

impl Triangle {
    pub fn new(point_a: Vec3f, point_b: Vec3f, point_c: Vec3f, albedo: Rgba<u8>) -> Triangle {
        Triangle {
            point_a,
            point_b,
            point_c,
            albedo,
        }
    }
}
// Möller-Trumbore intersection algorithm implementation.
impl Geometrical for Triangle {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit> {
        let epsilon = 1e-7;
        let edge1 = self.point_b - self.point_a;
        let edge2 = self.point_c - self.point_a;
        let h = ray.direction.cross(&edge2);
        let det = edge1.dot(&h);

        if det.abs() < epsilon {
            return None;
        }

        let f = 1.0 / det;

        let s = ray.origin - self.point_a;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        let point = ray.origin + t * ray.direction;
        let normal = (edge1.cross(&edge2) - self.point_a).normalized();
        Some(RaycastHit::new(point, normal, self.albedo))

    }
}
