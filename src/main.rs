mod utils;
use std::io::prelude::*;
use std::time::Instant;
use utils::vector::*;
use crate::utils::geometry::*;
use crate::utils::scene;
use crate::utils::renderer::render;

fn main() {
    // Multi-threading
    let num_threads = num_cpus::get() as u32;

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let fov = 20.0;
    let image_width = 900;
    let samples_per_pixel = 200;
    let max_depth: i32 = 5;
    let output_file = "default_scene_rel_mt.png";

    let mut image_height = (image_width as f32 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let scene_start = Instant::now();

    //let (world, camera) = scene::random_scene(aspect_ratio, fov);
    let (world, camera) = scene::default_scene(aspect_ratio, fov);

    let scene_duration = scene_start.elapsed();

    println!("Scene generation took: {:?}", scene_duration);

    println!("Beginning render!");
    println!("Image size: {}x{}", image_width, image_height);
    println!("Samples per pixel: {}", samples_per_pixel);
    println!("Max Bounces: {}", max_depth);
    println!("Number of Threads Threads: {}", num_threads);

    // Render
    render(
        camera,
        world,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        num_threads,
        output_file
    );

    let render_duration = scene_start.elapsed() - scene_duration;
    println!("Render took: {:?}", render_duration);
    println!("Output file: {}", output_file);

}

