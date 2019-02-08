mod vectors;
use vectors::*;

mod robjects;
#[allow(unused_imports)]
use robjects::*;

mod rays;
use rays::*;


#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() {
    Ray::new(Vec3f::zero(), Vec3f::unit_forward());
    println!("Rendering, lol.");
}
