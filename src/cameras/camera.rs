use crate::rays::Ray;

pub trait Camera: Sync + Send  {
    fn get_rays(&self) -> Vec<Ray>;
    fn get_fov_w(&self) -> u32;
    fn get_fov_h(&self) -> u32;
}
