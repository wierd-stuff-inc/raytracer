extern crate image;

use image::Rgba;

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
    let sphere = Sphere::new(
        Vec3f::new(256.0, 256.0, 256.0),
        100.0,
        Rgba {
            data: [255, 0, 0, 255],
        },
    );
    let sphere2 = Sphere::new(
        Vec3f::new(200.0, 128.0, 56.0),
        56.0,
        Rgba {
            data: [220, 0, 127, 255],
        },
    );
    let sphere3 = Sphere::new(
        Vec3f::new(150.0, 50.0, 30.0),
        40.0,
        Rgba {
            data: [64, 206, 92, 255],
        },
    );

    let triangle = Triangle::new(
        Vec3f::new(256.0, 300.0, 100.0),
        Vec3f::new(356.0, 156.0, 0.0),
        Vec3f::new(456.0, 300.0, 0.0),
        Rgba {
            data: [64, 206, 92, 255],
        },
    );
    let bg_color = Rgba {
        data: [255, 255, 255, 255],
    };

    let camera = Camera::new(Vec3f::zero(), width, height, 0.0, 0.0);
    let mut scene = Scene::new(camera, bg_color);
    scene.add_object(&sphere);
    scene.add_object(&sphere2);
    scene.add_object(&sphere3);
    scene.add_object(&triangle);

    scene.render()
}

fn save(img: image::DynamicImage, output_path: String) {
    img.save(output_path).unwrap();
}
