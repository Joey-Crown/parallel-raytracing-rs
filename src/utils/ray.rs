use crate::utils::vector::Vec3;
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn at(&self, t: f32) -> Vec3<f32> {
        self.origin + self.direction * t
    }
}
