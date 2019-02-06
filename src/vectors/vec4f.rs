#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::vec3f::Vec3f;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[cfg(test)]
impl Arbitrary for Vec4f {
    fn arbitrary<G: Gen>(g: &mut G) -> Vec4f {
        let x = f32::arbitrary(g);
        let y = f32::arbitrary(g);
        let z = f32::arbitrary(g);
        let w = f32::arbitrary(g);
        Vec4f{x, y, z, w}
    }
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f { x, y, z, w }
    }

    pub fn from_one(x: f32) -> Vec4f {
        Vec4f::new(x, x, x, x)
    }

    pub fn zero() -> Vec4f {
        Vec4f::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn unit() -> Vec4f {
        Vec4f::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn unit_forward() -> Vec4f {
        Vec4f::new(0.0, 0.0, 1.0, 0.0)
    }

    pub fn unit_backward() -> Vec4f {
        Vec4f::new(0.0, 0.0, -1.0, 0.0)
    }

    pub fn unit_right() -> Vec4f {
        Vec4f::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn unit_left() -> Vec4f {
        Vec4f::new(-1.0, 0.0, 0.0, 0.0)
    }

    pub fn unit_up() -> Vec4f {
        Vec4f::new(0.0, 1.0, 0.0, 0.0)
    }

    pub fn unit_down() -> Vec4f {
        Vec4f::new(0.0, -1.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f32 {
        let res = self.dot(&*self).sqrt();
        assert!(res + 1e-15 >= 0.0);
        res
    }

    pub fn squared_magnitude(&self) -> f32 {
        let res = self.dot(&*self);
        assert!(res + 1e-15 >= 0.0);
        res
    }

    pub fn normalized(&self) -> Vec4f {
        let length = self.magnitude();
        let res = Vec4f::new(
            self.x / length,
            self.y / length,
            self.z / length,
            self.w / length,
        );
        res
    }

    pub fn dot(&self, other: &Vec4f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: &Vec4f) -> Vec3f {
        // Ignoring w value.
        let a = Vec3f::new(self.x, self.y, self.z);
        let b = Vec3f::new(other.x, other.y, other.z);
        a.cross(&b)
    }

    pub fn comp(self, other: &Vec4f) -> f32 {
        self.dot(other) / self.magnitude()
    }

    pub fn project(self, other: &Vec4f) -> Vec4f {
        (self.dot(other) / self.squared_magnitude()) * self
    }
}

impl Add for Vec4f {
    type Output = Vec4f;

    fn add(self, other: Vec4f) -> Vec4f {
        Vec4f::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, other: Vec4f) -> Vec4f {
        Vec4f::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f32> for Vec4f {
    type Output = Vec4f;

    fn mul(self, other: f32) -> Vec4f {
        Vec4f::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Mul<Vec4f> for f32 {
    type Output = Vec4f;

    fn mul(self, other: Vec4f) -> Vec4f {
        other * self
    }
}

impl Neg for Vec4f {
    type Output = Vec4f;

    fn neg(self) -> Vec4f {
        Vec4f::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Div for Vec4f {
    type Output = Vec4f;

    fn div(self, rhs: Vec4f) -> Vec4f {
        Vec4f::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}
