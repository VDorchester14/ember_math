use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::core::vector4f::Vector4f;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Vector3f{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f{
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Vector3f{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    
    pub fn one() -> Self {
        Vector3f{
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    
    pub fn cross(&self, other: Self) -> Self {
        let x: f32 = (self.y * other.z) - (self.z * other.y);
        let y: f32 = (self.z * other.x) - (self.x * other.z);
        let z: f32 = (self.x * other.y) - (self.y * other.x);
        Vector3f::new(x, y, z)
    }

    pub fn magnitude_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        (x * x) + (y * y) + (z * z)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Vector3f::new(self.x / m, self.y / m, self.z / m)
    }

    pub fn scale(&self, s: f32) -> Self {
        Vector3f::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn extend(&self, w: f32) -> Vector4f {
        Vector4f::new(self.x, self.y, self.z, w)
    }

    pub fn angle_between_rad(&self, other: Self) -> f32 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
}

impl Add for Vector3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vector3f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl Sub for Vector3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl SubAssign for Vector3f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl From<Vector3f> for [f32; 3] {
    fn from(vec: Vector3f) -> [f32; 3] {
        [vec.x, vec.y, vec.z]
    }
}


impl PartialEq for Vector3f {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}