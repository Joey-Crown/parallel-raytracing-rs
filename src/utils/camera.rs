use image::{Rgb, RgbImage};
use rand::{Rng, thread_rng};
use crate::utils::color::Color;
use crate::utils::geometry::*;
use crate::utils::material;
use crate::utils::material::*;
use crate::utils::ray::Ray;
use crate::utils::vector::Vec3;

pub struct Camera {
    origin: Vec3<f32>,
    upper_left_corner: Vec3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    u: Vec3<f32>,
    v: Vec3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3<f32>,
        direction: Vec3<f32>,
        up: Vec3<f32>, vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Self {
        let theta = vfov.to_radians();
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - direction).normalise();
        let u = up.cross(w).normalise();
        let v = w.cross(u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * -viewport_height * focus_dist;
        let upper_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            origin,
            upper_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.upper_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset
        }
    }

    pub fn render(&self,
                  world: &HittableList,
                  image_width: u32,
                  image_height: u32,
                  samples_per_pixel: u32,
                  max_depth: i32,
                  output_file: &str
    ) -> () {
        let mut img = RgbImage::new(image_width, image_height);

        let pixel_delta_horizontal = self.horizontal / (image_width as f32);
        let pixel_delta_vertical = self.vertical / (image_height as f32);
        let pixel100_location = self.upper_left_corner + pixel_delta_horizontal + pixel_delta_vertical * 0.5;

        let mut rng = thread_rng();
        for x in 0..image_width {
            if (image_width - x - 1) % 50 == 0 {
                println!("Scanlines remaining: {}", image_width - x - 1);
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
        }

        img.save(output_file).unwrap();
        println!("Finished Rendering!");
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList, samples: u32) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f32::INFINITY) {
        return Color::from_vec3_float((rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5, samples);
    }

    let unit_direction = ray.direction.normalise();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::from_vec3_float(
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) +
            Vec3::new(0.5, 0.7, 1.0) * t, samples
    )
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
