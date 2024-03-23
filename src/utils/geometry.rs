use std::rc::Rc;
use std::sync::Arc;
use crate::utils::color::Color;
use crate::utils::ray::Ray;
use crate::utils::vector::{Dot, Vec3};
use crate::utils::material;
use crate::utils::material::Material;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub material: Option<Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3<f32>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable + Sync + Send>>) -> Self {
        HittableList { objects: list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in self.objects.iter() {
            if let Some(rec) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_record = Some(rec);
            }
        }
        temp_record
    }
}

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - (self.radius * self.radius);


        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root <= t_min || t_max <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= t_min || t_max <= root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            p,
            material: Some(self.material.clone()),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius as f32;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}