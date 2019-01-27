extern crate image;

use image::{GenericImage, Rgba};

mod vectors;
use vectors::*;

mod geometrical;
use geometrical::*;

mod misc;
use misc::*;

fn main() {
    let width = 512;
    let height = 512;
    let output_path = "./output.png";
    let img = render(width, height);
    save(img, output_path.to_string());
}

fn render(width: u32, height: u32) -> image::DynamicImage {
    let mut img = image::DynamicImage::new_rgb8(width, height);

    let sphere = Sphere::new(
        Vec3f::new(256.0, 256.0, 256.0),
        512.0,
        Rgba {
            data: [255, 0, 0, 255],
        },
    );
    let bg_color = Rgba {
        data: [255, 255, 255, 255],
    };

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
