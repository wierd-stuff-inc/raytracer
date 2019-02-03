extern crate image;
extern crate rayon;

use image::Rgba;

mod vectors;
use vectors::*;

mod geometry;
use geometry::*;

mod cameras;
use cameras::Orthographic;

mod rays;

mod scene;
use scene::Scene;

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
        Vec3f::new(256.0, 500.0, 0.0),
        Vec3f::new(356.0, 356.0, 400.0),
        Vec3f::new(456.0, 500.0, 0.0),
        Rgba {
            data: [149, 226, 104, 255],
        },
    );

    let triangle2 = Triangle::new(
        Vec3f::new(256.0, 500.0, 0.0),
        Vec3f::new(156.0, 356.0, 0.0),
        Vec3f::new(356.0, 356.0, 0.0),
        Rgba {
            data: [149, 226, 104, 255],
        },
    );
    let bg_color = Rgba {
        data: [226, 182, 104, 255],
    };

    let camera = Orthographic::new(Vec3f::zero(), width, height, 0.0, 0.0);
    let mut scene = Scene::new(camera, bg_color);
    scene.add_object(&sphere);
    scene.add_object(&sphere2);
    scene.add_object(&sphere3);
    scene.add_object(&triangle);
    scene.add_object(&triangle2);
    scene.render()
}

fn save(img: image::DynamicImage, output_path: String) {
    img.save(output_path).unwrap();
}
