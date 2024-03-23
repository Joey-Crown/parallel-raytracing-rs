use std::ops::{Neg, Add, Mul, Div, Sub, Rem, Range};
use num::Float;
use rand::Rng;

pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vec3<f32> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random(r: Range<f32>) -> Vec3<f32> {
        let mut rng = rand::thread_rng();

        Vec3 { 
            x: rng.gen_range(r.clone()),
            y: rng.gen_range(r.clone()), 
            z: rng.gen_range(r.clone())
        }
    }

    pub fn random_in_unit_sphere() -> Vec3<f32> {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3<f32> {
        let mut rng = rand::thread_rng();

        loop {
            let v = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0
            );
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, n: Vec3<f32>) -> Vec3<f32> {
        *self - n * 2.0 * self.dot(n)
    }

    pub fn refract(&self, n: Vec3<f32>, etai_over_etat: f32) -> Vec3<f32> {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn cross(&self, other: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Copy + Mul<Output = A>, A: Add<Output = A> + Float> Vec3<T> {
    pub fn length(&self) -> A {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T: Copy + Mul<Output = A>, A: Add<Output = A> + Float> Vec3<T> {
    pub fn length_squared(&self) -> A {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Copy + Mul<Output = A> + Div<A>, A: Add<Output = A> + Float + Copy> Vec3<T> {
    pub fn normalise(self) -> Vec3<<T as Div<A>>::Output> {
        self / self.length()
    }
}

impl<T: Mul<U, Output = S>, S: Add<Output = S>, U: Copy> Dot<Vec3<U>> for Vec3<T> {
    type Output = <S as Add>::Output;

    fn dot (self, other: Vec3<U>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}


impl <T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg> ::Output>;
    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl <T: Add<S>, S: Copy> Add<Vec3<S>> for Vec3<T> {
    type Output = Vec3<<T as Add<S>>::Output>;
    fn add(self, other: Vec3<S>) -> Self::Output {
        Self::Output{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: Sub<S>, S: Copy> Sub<Vec3<S>> for Vec3<T> {
    type Output = Vec3<<T as Sub<S>>::Output>;

    fn sub(self, other: Vec3<S>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<S>, S: Copy> Mul<S> for Vec3<T> {
    type Output = Vec3<<T as Mul<S>>::Output>;

    fn mul(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Div<S>, S: Copy> Div<S> for Vec3<T> {
    type Output = Vec3<<T as Div<S>>::Output>;

    fn div(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: Rem<S>, S: Copy> Rem<S> for Vec3<T> {
    type Output = Vec3<<T as Rem<S>>::Output>;

    fn rem(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x % scalar,
            y: self.y % scalar,
            z: self.z % scalar,
        }
    }
}

