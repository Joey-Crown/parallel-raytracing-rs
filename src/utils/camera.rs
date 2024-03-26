use crate::utils::ray::Ray;
use crate::utils::vector::Vec3;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3<f32>,
    pub upper_left_corner: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>,
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
}


