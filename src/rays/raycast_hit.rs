use image::{Rgba};
use crate::vectors::*;

#[derive(Debug, Copy, Clone)]
pub struct RaycastHit {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub albedo: Rgba<u8>,
}

impl RaycastHit {
    pub fn new(point: Vec3f, normal: Vec3f, albedo: Rgba<u8>) -> RaycastHit {
        RaycastHit {
            point,
            normal,
            albedo,
        }
    }
}
