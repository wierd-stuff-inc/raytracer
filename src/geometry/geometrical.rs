extern crate image;
use crate::rays::Ray;
use crate::rays::RaycastHit;

pub trait Geometrical: std::fmt::Debug {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit>;
}
