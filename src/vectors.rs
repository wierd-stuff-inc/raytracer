use std::ops::{Add, Mul, Sub};

// VECTORS
//_____________________________________________________________________

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn from_one(x: f32) -> Vec3f {
        Vec3f::new(x, x, x)
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(&*self).sqrt()
    }

    pub fn normalized(&self) -> Vec3f {
        let length = self.magnitude();

        Vec3f::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, other: f32) -> Vec3f {
        Vec3f::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        other * self
    }
}
