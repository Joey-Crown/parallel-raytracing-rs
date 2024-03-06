use crate::utils::vector::Vec3;
use num::clamp; 

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub(crate) value: Vec3<u8>,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            value: Vec3 { x: r, y: g, z: b }
        }
    }

    pub fn from_vec3_float(value: Vec3<f32>, samples: u32) -> Self {
        Color {
            value: Vec3 {
                x: (256.0 * (value.x / (samples as f32)).sqrt().clamp(0.0, 0.999)) as u8,
                y: (256.0 * (value.y / (samples as f32)).sqrt().clamp(0.0, 0.999)) as u8,
                z: (256.0 * (value.z / (samples as f32)).sqrt().clamp(0.0, 0.999)) as u8,
            }
        }
    }

    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Color {
            value: Vec3 {
                x: (255.999 * r) as u8,
                y: (255.999 * g) as u8,
                z: (255.999 * b) as u8,
            }
        }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r(), self.g(), self.b()]
    }

    pub fn r(&self) -> u8 {
        self.value.x
    }

    pub fn g(&self) -> u8 {
        self.value.y
    }

    pub fn b(&self) -> u8 {
        self.value.z
    }
}
