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

impl Geometrical for Triangle {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit> {
        // Calculating triangle normal.
        let triangle_plane_norm =
            (self.point_b - self.point_a).cross(&(self.point_c - self.point_a));

        let norm_dot_ray_dir = triangle_plane_norm.dot(&ray.direction);
        //Check if ray and triangle are parallel; If true => No intersection;

        if norm_dot_ray_dir.abs() < 1e-7 {
            return None;
        }
        let d = triangle_plane_norm.dot(&self.point_a);
        let t = (triangle_plane_norm.dot(&ray.direction) + d) / norm_dot_ray_dir;
        // If t is less than zero => ray is behind triangle => No intersection;
        if t < 0.0 {
            return None;
        }

        let intersection_point = ray.origin + ray.direction * t;

        // tests if intersection_point is on the left side of each one of the triangle's edges
        let a = (self.point_b - self.point_a).cross(&(intersection_point - self.point_a));
        if triangle_plane_norm.dot(&a) < 0.0 {
            return None;
        }

        let b = (self.point_c - self.point_b).cross(&(intersection_point - self.point_b));
        if triangle_plane_norm.dot(&b) < 0.0 {
            return None;
        }

        let c = (self.point_a - self.point_c).cross(&(intersection_point - self.point_c));
        if triangle_plane_norm.dot(&c) < 0.0 {
            return None;
        }

        //todo: Fix normal. I think, something goes wrong.
        let normal = (intersection_point - triangle_plane_norm).normalized();
        Some(RaycastHit::new(
            intersection_point,
            normal,
            self.albedo,
        ))
    }
}
