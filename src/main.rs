extern crate image;

use std::ops::{Add, Sub, Mul};

use image::{GenericImage, Rgba};


// VECTORS
//_____________________________________________________________________

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f {
    fn new(x: f32, y: f32, z:f32) -> Vec3f {
        Vec3f{x, y, z}
    }

    fn from_one(x: f32) -> Vec3f {
        Vec3f::new(x, x, x)
    }

    fn magnitude(&self) -> f32 {
        self.dot(&*self).sqrt()
    }

    fn normalized(&self) -> Vec3f {
        let length = self.magnitude();

        Vec3f::new(self.x / length, self.y / length, self.z / length)
    }

    fn dot(&self, other : &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Vec3f)->Vec3f {
        Vec3f{
            x:self.y * other.z - self.z * other.y,
            y:self.z * other.x - self.x * other.z,
            z:self.x * other.y - self.y * other.x
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

// GEOMETRIC PRIMITIVES
//_____________________________________________________________________
trait Geometrical : std::fmt::Debug {
    fn intersect_ray(&self, ray: Ray) -> Option<Rgba<u8>>;
}

#[derive(Debug, PartialEq)]
struct Sphere {
    position: Vec3f,
    radius: f32,
    color: Rgba<u8>,
}

impl Sphere {
    fn new(n_position: Vec3f, n_radius: f32, n_color: Rgba<u8>) -> Sphere {
        Sphere{position: n_position, radius: n_radius, color: n_color}
    }
}

impl Geometrical for Sphere {
    fn intersect_ray(&self, ray: Ray) -> Option<Rgba<u8>> {
        // Vector from ray origin to sphere center
        let l = self.position - ray.origin;
        let foo = ray.direction;
        // Define whether sphere is visible or not
        let dot = l.dot(&foo);
        if dot <= 0.0 {
            // Sphere is behind the ray origin, so return None
            return None;
        }
        // Sphere is in a FOV, so computing projection on a Ray
        // Finding a distance between sphere center and Ray
        let proj = l * (dot / l.magnitude().powf(2.0));
        let distance = (proj - self.position).magnitude();
        // If distance is less than radius, Ray is intersecting Sphere
        if distance <= self.radius {
            Some(self.color)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cube {
    position: Vec3f,
    size: f32,
    color: Rgba<u8>,
}

impl Cube {
    fn new(n_position: Vec3f, n_size: f32, n_color: Rgba<u8>) -> Cube {
        Cube{position: n_position, size: n_size, color: n_color}
    }
}

impl Geometrical for Cube {
    fn intersect_ray(&self, ray: Ray) -> Option<Rgba<u8>> {
        unimplemented!()
    }
}

// MISC.
//_____________________________________________________________________

#[derive(Debug)]
struct Camera {
    position: Vec3f,
    fov_w: u32,
    fov_h: u32,
    near_clipping_plane: f32,
    far_clipping_plane: f32
}

#[derive(Debug, Clone)]
struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

impl Ray {
    fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        Ray { origin, direction }
    }

    fn normalized(self) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction.normalized()
        }
    }
}

#[derive(Debug)]
struct Scene <'a> {
    // camera: Camera,
    objects: Vec<&'a Geometrical>,
}

// Renders
//_____________________________________________________________________

fn main() {
    let width = 512;
    let height = 512;
    let output_path = "./output.png";
    let img = render(width, height);
    save(img, output_path.to_string());
}

fn render(width: u32, height: u32) -> image::DynamicImage {
    let mut img = image::DynamicImage::new_rgb8(width, height);

    let sphere = Sphere::new(Vec3f::new(256.0, 256.0, 155.0), 50.0, Rgba { data: [255, 0, 0, 0] });
    let bg_color = Rgba { data: [0, 255, 0, 0] };

    for x in 0..width {
        for y in 0..height {
            let ray = Ray {
                origin: Vec3f::new(x as f32, y as f32, 0.0),
                direction: Vec3f::new(0.0, 0.0, 1.0),
            };
            if let Some(color) = sphere.intersect_ray(ray) {
                img.put_pixel(x, y, color);
            } else {
                img.put_pixel(x, y, bg_color);
            }

        }
    }

    img
}

fn save(img: image::DynamicImage, output_path: String) {
    img.save(output_path).unwrap();
}
