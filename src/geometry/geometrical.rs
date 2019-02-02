extern crate image;

use crate::misc::RaycastHit;

use crate::misc::Ray;

pub trait Geometrical: std::fmt::Debug {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit>;
}
