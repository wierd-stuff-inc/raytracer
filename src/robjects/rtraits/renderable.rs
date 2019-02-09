use crate::rays::Ray;
use crate::rays::RaycastHit;

pub trait Renderable: std::fmt::Debug + Sync + Send {
    fn intersect_ray(&self, ray: Ray) -> Option<RaycastHit>;
}
