use crate::utils::geometry::HitRecord;
use crate::utils::ray::Ray;
use crate::utils::vector::{Dot, Vec3};
use rand::{Rng, thread_rng};

#[derive(Clone)]
pub enum Material {
    Lambertian {
        albedo: Vec3<f32>,
    },
    Metal {
        albedo: Vec3<f32>,
        fuzz: f32,
    },
    Dielctric {
        ir: f32,
    },
}

pub fn scatter(material: &Material, r_in: &Ray, rec: &HitRecord) -> (Vec3<f32>, Ray, bool) {
    match material {
        Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalise();
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }
            let scattered = Ray::new(rec.p, scatter_direction);
            (*albedo, scattered, true)
        }
        Material::Metal { albedo, fuzz } => {
            let reflected = r_in.direction.reflect(rec.normal);
            let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * *fuzz);
            let should_scatter = scattered.direction.dot(rec.normal) > 0.0;
            (*albedo, scattered, should_scatter)
        }
        Material::Dielctric { ir } => {
            let refraction_ratio = if rec.front_face { 1.0 / *ir } else { *ir };
            let unit_direction = r_in.direction.normalise();

            let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let mut rng = thread_rng();
            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let will_reflect = rng.gen::<f32>() < schlick(cos_theta, refraction_ratio);

            let direction = if cannot_refract || will_reflect {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

            let scattered = Ray::new(rec.p, direction);

            (Vec3::new(1.0, 1.0, 1.0), scattered, true)
        }
    }
}

pub fn schlick(cosine: f32, ir: f32) -> f32 {
    let r0 = ((1.0 - ir) / (1.0 + ir)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}