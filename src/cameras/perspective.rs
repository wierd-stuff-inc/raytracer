use crate::vectors::*;
use super::camera::Camera;
use crate::rays::Ray;

use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Perspective {
    pub position: Vec3f,
    pub fov_w: u32,
    pub fov_h: u32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
}
impl Perspective {
    pub fn new(
        position: Vec3f,
        fov_w: u32,
        fov_h: u32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
    ) -> Perspective {
        Perspective{
            position,
            fov_w,
            fov_h,
            near_clipping_plane,
            far_clipping_plane
        }
    }
}

impl Camera for Perspective{
    fn get_rays(&self) -> Vec<Ray>{
        Vec::new()
    }
    fn get_fov_h(&self) -> u32{
        self.fov_h
    }
    fn get_fov_w(&self) -> u32{
        self.fov_w
    }
}
