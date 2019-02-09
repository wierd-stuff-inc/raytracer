use crate::rays::Ray;
use crate::robjects::rtraits::Camera;
use crate::vectors::Vec3f;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Orthographic {
    pub position: Vec3f,
    pub fov_w: u32,
    pub fov_h: u32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
}

impl Orthographic {
    fn new(
        position: Vec3f,
        fov_w: u32,
        fov_h: u32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
    ) -> Orthographic {
        Orthographic{
            position,
            fov_w,
            fov_h,
            near_clipping_plane,
            far_clipping_plane
        }
    }
}

impl Camera for Orthographic {
    fn cast_rays() -> Vec<Ray> {
        Vec::new()
    }
}
