#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

#[cfg(test)]
impl Arbitrary for Vec2f {
    fn arbitrary<G: Gen>(g: &mut G) -> Vec2f {
        let x = f32::arbitrary(g);
        let y = f32::arbitrary(g);
        Vec2f{x, y}
    }
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }

    pub fn from_one(x: f32) -> Vec2f {
        Vec2f::new(x, x)
    }

    pub fn zero() -> Vec2f {
        Vec2f::new(0.0, 0.0)
    }

    pub fn unit() -> Vec2f {
        Vec2f::new(1.0, 1.0)
    }

    pub fn unit_right() -> Vec2f {
        Vec2f::new(1.0, 0.0)
    }

    pub fn unit_left() -> Vec2f {
        Vec2f::new(-1.0, 0.0)
    }

    pub fn unit_up() -> Vec2f {
        Vec2f::new(0.0, 1.0)
    }

    pub fn unit_down() -> Vec2f {
        Vec2f::new(0.0, -1.0)
    }

    pub fn magnitude(&self) -> f32 {
        let res = self.dot(&*self).sqrt();
        res
    }

    pub fn squared_magnitude(&self) -> f32 {
        let res = self.dot(&*self);
        res
    }

    pub fn normalized(&self) -> Vec2f {
        let length = self.magnitude();
        let res = Vec2f::new(self.x / length, self.y / length);
        res
    }

    pub fn dot(&self, other: &Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: &Vec2f) -> f32 {
        // todo:Decide which implementation we should choose.
        ///////
        // First one
        ///////
        // let a = Vec3f::new(self.x, self.y, 0.0);
        // let b = Vec3f::new(other.x, other.y, other.z);
        // a.cross(b)
        ///////
        // Second
        ///////
        self.x * other.y - self.y * other.x
    }

    pub fn comp(self, other: &Vec2f) -> f32 {
        self.dot(other) / self.magnitude()
    }

    pub fn project(self, other: &Vec2f) -> Vec2f {
        (self.dot(other) / self.squared_magnitude()) * self
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, other: f32) -> Vec2f {
        Vec2f::new(self.x * other, self.y * other)
    }
}

impl Mul<Vec2f> for f32 {
    type Output = Vec2f;

    fn mul(self, other: Vec2f) -> Vec2f {
        other * self
    }
}

impl Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Vec2f {
        Vec2f::new(-self.x, -self.y)
    }
}

impl Div for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Vec2f {
        Vec2f::new(self.x / rhs.x, self.y / rhs.y)
    }
}
