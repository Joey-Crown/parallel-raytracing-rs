use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Instant;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{Rng, thread_rng};
use crate::utils::camera::{Camera};
use crate::utils::color::Color;
use crate::utils::geometry::{Hittable, HittableList};
use crate::utils::material::scatter;
use crate::utils::ray::Ray;
use crate::utils::vector::Vec3;

pub fn render(camera: Camera,
              world: HittableList,
              image_width: u32,
              image_height: u32,
              samples_per_pixel: u32,
              max_depth: i32,
              num_threads: u32,
              output_file: &str
    ) -> () {
    let pixel_delta_horizontal = camera.horizontal / (image_width as f32);
    let pixel_delta_vertical = camera.vertical / (image_height as f32);
    let pixel100_location = camera.upper_left_corner + pixel_delta_horizontal + pixel_delta_vertical * 0.5;

    let timer = Instant::now();
    //let (tx, rx) = mpsc::channel();
    let mut threads = vec![];

    let arc_world = Arc::new(world);
    let arc_img = Arc::new(Mutex::new(RgbImage::new(image_width, image_height)));

    for i in 0..num_threads {
        let inner_world = arc_world.clone();
        let inner_img = arc_img.clone();

        threads.push(std::thread::spawn(move || {
            let start = i * image_width / num_threads;
            let end = if i == num_threads - 1 {
                image_width
            } else {
                (i as u32 + 1) * image_width / num_threads
            };

            let mut sub_img = ImageBuffer::new(end - start, image_height);

            let mut rng = thread_rng();
            for x in start..end {
                for y in 0..image_height {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    //Multisampling
                    for _ in 0..samples_per_pixel {
                        let ru: f32 = rng.gen();
                        let rv: f32 = rng.gen();

                        let u = ((x as f32) + ru) / ((image_width-1) as f32);
                        let v = ((y as f32) + rv) / ((image_height-1) as f32);

                        let ray = camera.get_ray(u, v);
                        pixel_color = ray_color_vec3_float(&ray, &inner_world, max_depth) + pixel_color;
                    }
                    //Average colors
                    let final_color = Color::from_vec3_float(pixel_color, samples_per_pixel);
                    sub_img.put_pixel(x - start, y, Rgb(final_color.to_rgb()));
                }

            }

            let mut img_data = inner_img.lock().unwrap();
            for x in start..end {
                for y in 0..image_height {
                    img_data.put_pixel(x, y, *sub_img.get_pixel((x - start) as u32, y));
                }
            }
        }));
    }

    for _ in 0..num_threads {

    }

    for thread in threads {
        thread.join().unwrap();
    }

    /*let mut rng = thread_rng();
    for x in 0..image_width {
        if (image_width - x - 1) % 50 == 0 {
            let elapsed = start.elapsed();
            println!("Scanlines remaining: {} (Current Time: {:?})", image_width - x - 1, elapsed);
        }
        for y in 0..image_height {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            //Multisampling
            for _ in 0..samples_per_pixel {
                let ru: f32 = rng.gen();
                let rv: f32 = rng.gen();

                let u = ((x as f32) + ru) / ((image_width-1) as f32);
                let v = ((y as f32) + rv) / ((image_height-1) as f32);

                let ray = self.get_ray(u, v);
                pixel_color = ray_color_vec3_float(&ray, &world, max_depth) + pixel_color;
            }
            //Average colors
            let final_color = Color::from_vec3_float(pixel_color, samples_per_pixel);
            img.put_pixel(x, y, Rgb(final_color.to_rgb()));
        }
    }*/

    let final_img = arc_img.lock().unwrap();
    final_img.save_with_format(output_file, image::ImageFormat::Png).unwrap();
    println!("Finished Rendering!");
}

//Returns the ray's color but in the form of an f32 vec so it can be summed and then passed to ray_color for an anti-aliased clamped average
pub fn ray_color_vec3_float(ray: &Ray, world: &HittableList, depth: i32) -> Vec3<f32> {
    world.hit(ray, 0.001, f32::INFINITY).and_then(|rec| {
        if rec.material.is_some() {
            return rec.material.clone().map(|mat| {
                if depth >= 0 {
                    let (attenuation, scattered, should_scatter) = scatter(&mat, ray, &rec);
                    if should_scatter {
                        let pixel = ray_color_vec3_float(&scattered, world, depth - 1);
                        return Vec3::new(attenuation.x * pixel.x, attenuation.y * pixel.y, attenuation.z * pixel.z);
                    }
                }
                Vec3::new(0.0, 0.0, 0.0)
            })
        } else {
            None
        }
    }).unwrap_or_else(|| {
        let unit_direction = ray.direction.normalise();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    })
}