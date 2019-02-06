use crate::vectors::Vec4f;
#[cfg(test)]
extern crate quickcheck;

#[quickcheck]
fn magnitude(x: Vec4f) -> bool {
    x.magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn squared_magnitude(x: Vec4f) -> bool {
    x.squared_magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn normalized(x: Vec4f) -> bool{
    (x.normalized().magnitude() - 1.0).abs() <= 1e-6
}
