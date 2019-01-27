extern crate image;

use image::Rgba;

use crate::misc::Ray;
use crate::vectors::*;

pub trait Geometrical: std::fmt::Debug {
    fn intersect_ray(&self, ray: Ray) -> Option<Rgba<u8>>;
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
    fn intersect_ray(&self, ray: Ray) -> Option<Rgba<u8>> {
        // Vector from ray origin to sphere center
        let l = self.position - ray.origin;
        let foo = ray.origin + ray.direction;
        // Define whether sphere is visible or not
        let dot = l.dot(&foo);
        if dot <= 0.0 {
            // Sphere is behind the ray origin, so return None
            return None;
        }
        // Sphere is in a FOV, so computing projection on a Ray

        // Finding a distance between sphere center and Ray
        // https://gamedev.ru/tip/?id=42

        let proj = l * (dot / l.squared_magnitude());
        let distance = (proj - self.position).magnitude();

        //This is a simplest, but more computational-heavy solution
        // let norm_l = l.normalized();
        // let proj = norm_l * norm_l.dot(&foo);
        // let distance = (proj - self.position).magnitude();

        // If distance is less than radius, Ray is intersecting Sphere
        if distance <= self.radius {
            Some(self.color)
        } else {
            None
        }
    }
}
