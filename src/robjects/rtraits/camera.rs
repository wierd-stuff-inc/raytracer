use crate::rays::Ray;

pub trait Camera: std::fmt::Debug {
    fn cast_rays() -> Vec<Ray>;
}
