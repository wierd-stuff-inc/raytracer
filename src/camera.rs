use crate::vectors::*;

use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3f,
    pub fov_w: u32,
    pub fov_h: u32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
}

impl Camera {
    pub fn new(
        position: Vec3f,
        fov_w: u32,
        fov_h: u32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
    ) -> Camera {
        Camera{
            position,
            fov_w,
            fov_h,
            near_clipping_plane,
            far_clipping_plane
        }
    }
}
