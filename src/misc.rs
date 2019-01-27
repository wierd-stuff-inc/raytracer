use crate::geometrical::*;
use crate::vectors::*;

#[derive(Debug)]
pub struct Camera {
    position: Vec3f,
    fov_w: u32,
    fov_h: u32,
    near_clipping_plane: f32,
    far_clipping_plane: f32,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        Ray { origin, direction }
    }
}

#[derive(Debug)]
pub struct Scene<'a> {
    camera: Camera,
    objects: Vec<&'a Geometrical>,
}
