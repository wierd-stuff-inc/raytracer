use crate::rays::RaycastHit;
use crate::robjects::rtraits::Renderable;
use crate::robjects::rtraits::SceneObject;
use image::Rgba;

use crate::rays::Ray;
use crate::vectors::*;

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

impl SceneObject for Triangle {

}

// Möller-Trumbore intersection algorithm implementation.
impl Renderable for Triangle {
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
