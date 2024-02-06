use std::ops::{Neg, Add, Mul, Div, Sub, Rem};
use num::Float;

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

