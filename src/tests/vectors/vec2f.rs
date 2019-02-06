use crate::vectors::Vec2f;
#[cfg(test)]
extern crate quickcheck;

#[quickcheck]
fn magnitude(x: Vec2f) -> bool {
    x.magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn squared_magnitude(x: Vec2f) -> bool {
    x.squared_magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn normalized(x: Vec2f) -> bool{
    (x.normalized().magnitude() - 1.0).abs() <= 1e-6
}
