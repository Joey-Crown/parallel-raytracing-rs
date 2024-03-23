mod utils;

use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use utils::vector::*;
use utils::color::*;
use crate::utils::geometry::*;
use crate::utils::camera::*;
use crate::utils::material::*;
use crate::utils::material::Material;
use crate::utils::scene;

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let fov = 20.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth: i32 = 5;
    let output_file = "random_scene.png";

    let mut image_height = (image_width as f32 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let (world, camera) = scene::random_scene(aspect_ratio, fov);

    // Render
    camera.render(
        &world,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        output_file
    );

}

