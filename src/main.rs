mod utils;

use std::io::prelude::*;
use utils::vector::*;
use utils::color::*;
use crate::utils::geometry::*;
use crate::utils::camera::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;

    let mut image_height = (image_width as f32 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Color::new(255,0,0))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Color::new(0,255,0)))

    ]);

    // Camera
    let camera = Camera::new();

    // Render
    camera.render(&world, image_width, image_height, samples_per_pixel);

}
