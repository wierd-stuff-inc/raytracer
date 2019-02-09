use crate::robjects::rtraits::Camera;
use crate::robjects::rtraits::Renderable;
use image::Rgba;

#[derive(Debug)]
pub struct Scene<'a, Rec: Camera> {
    pub camera: Rec,
    pub objects: Vec<&'a (Renderable + Send + Sync)>,
    pub background_color: Rgba<u8>,
}

impl<'a, Rec: Camera> Scene<'a, Rec> {
    pub fn new(camera: Rec, background_color: Rgba<u8>) -> Scene<'a, Rec> {
        let objects: Vec<&'a (Renderable + Send + Sync)> = Vec::new();
        Scene {
            camera,
            objects,
            background_color,
        }
    }
    pub fn add_object(&mut self, object: &'a (Renderable + Send + Sync)) {
        self.objects.push(object);
    }
}
