use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};
use crate::core::vector3f::Vector3f;

use serde::{
    Serialize,
    Deserialize,
};
use bevy_reflect::{
    Reflect,
    FromReflect
};
use bevy_ecs::prelude::Resource;


#[derive(Debug, Copy, Clone, Serialize, Deserialize, Reflect, FromReflect, Resource)]
#[repr(C)]
pub struct Vector4f{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vector4f {
    fn default() -> Self {
        Vector4f::zero()
    }
}

impl Vector4f{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4f{
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn zero() -> Self {
        Vector4f{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    
    pub fn one() -> Self {
        Vector4f{
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn magnitude_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;
        (x * x) + (y * y) + (z * z) + (w * w)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Vector4f::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    pub fn scale(&self, s: f32) -> Self {
        Vector4f::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }

    pub fn truncate(&self) -> Vector3f {
        Vector3f::new(self.x, self.y, self.z)
    }

}

impl Add for Vector4f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl AddAssign for Vector4f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        };
    }
}

impl Sub for Vector4f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl SubAssign for Vector4f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        };
    }
}

impl From<Vector4f> for [f32; 4] {
    fn from(vec: Vector4f) -> [f32; 4] {
        [vec.x, vec.y, vec.z, vec.w]
    }
}


impl PartialEq for Vector4f {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}