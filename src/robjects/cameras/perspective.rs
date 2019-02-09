use crate::robjects::rtraits::Camera;
use crate::rays::Ray;
use crate::vectors::Vec3f;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Perspective {
    pub position: Vec3f,
    pub fov_w: u32,
    pub fov_h: u32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
}

impl Perspective {
    fn new(
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

impl Camera for Perspective {
    fn cast_rays() -> Vec<Ray>{
        Vec::new()
    }
}
