use crate::vectors::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        assert_ne!(origin, direction);
        Ray { origin, direction }
    }
}
