use crate::geometrical::*;
use crate::vectors::*;
use image::{Rgba, GenericImage};
use std::f32;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    position: Vec3f,
    fov_w: u32,
    fov_h: u32,
    near_clipping_plane: f32,
    far_clipping_plane: f32,
}

impl Camera {
    pub fn new(
        position: Vec3f,
        fov_w: u32,
        fov_h: u32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
    ) -> Camera {
        Camera{
            position,
            fov_w,
            fov_h,
            near_clipping_plane,
            far_clipping_plane
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        assert_ne!(origin, direction);
        Ray { origin, direction }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RaycastHit {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub albedo: Rgba<u8>,
}

impl RaycastHit {
    pub fn new(point: Vec3f, normal: Vec3f, albedo: Rgba<u8>) -> RaycastHit {
        RaycastHit {
            point,
            normal,
            albedo,
        }
    }
}

#[derive(Debug)]
pub struct Scene<'a> {
    pub camera: Camera,
    pub objects: Vec<&'a (Geometrical + Send + Sync)>,
    pub background_color: Rgba<u8>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera, background_color: Rgba<u8>) -> Scene<'a> {
        let objects: Vec<&'a (Geometrical + Send + Sync)> = Vec::new();
        Scene {
            camera,
            objects,
            background_color,
        }
    }

    pub fn add_object(&mut self, object: &'a (Geometrical + Send + Sync)) {
        self.objects.push(object);
    }

    pub fn set_objects(&mut self, objects: Vec<&'a (Geometrical + Send + Sync)>) {
        self.objects = objects;
    }

    pub fn render(&self) -> image::DynamicImage {
        let mut img = image::DynamicImage::new_rgb8(self.camera.fov_w, self.camera.fov_h);

        let objects_lock = RwLock::new(&self.objects);
        let objects_arc = Arc::new(objects_lock);

        let (sender, receiver) = channel();

        (0..self.camera.fov_w).into_par_iter().for_each_with(sender, |s, x| {
            for y in 0..self.camera.fov_h {
                let objects_rw = Arc::clone(&objects_arc);
                let objs = objects_rw.read().unwrap();

                let ray = Ray::new(Vec3f::new(x as f32, y as f32, 0.0), Vec3f::unit_forward());

                // Defining inital z-buffer value (depth of last rendered pixel)
                let mut z_buffer = f32::INFINITY;
                let mut final_color = self.background_color;
                for obj in *objs {
                    if let Some(hit) = obj.intersect_ray(ray) {
                        // Checking if current pixel is closer to camera than last rendered
                        let hit_zbuf = (hit.point - ray.origin).magnitude();
                        if  hit_zbuf < z_buffer {
                            // Assuming that ray direction is also a light direction.
                            // Calculating the illuminati(on). Confirmed.
                            let coeff = hit.normal.dot(&ray.direction);
                            let rgba = hit.albedo.data;
                            let color = Rgba{data: [
                                ((rgba[0] as f32) * coeff) as u8,
                                ((rgba[1] as f32) * coeff) as u8,
                                ((rgba[2] as f32) * coeff) as u8,
                                255]
                            };
                            final_color = color;
                            z_buffer = hit_zbuf;
                        }
                    }
                }

                s.send((x, y, final_color));
            }
        });

        receiver.iter().for_each( |(x, y, color)| {
            img.put_pixel(x, y, color);
        });

        img
    }
}
