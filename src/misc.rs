use crate::geometrical::*;
use crate::vectors::*;
use image::Rgba;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    position: Vec3f,
    fov_w: u32,
    fov_h: u32,
    near_clipping_plane: f32,
    far_clipping_plane: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        assert_ne!(origin, direction);
        Ray { origin, direction }
    }
}

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

#[derive(Debug)]
pub struct Scene<'a> {
    camera: Camera,
    objects: Vec<&'a Geometrical>,
}
