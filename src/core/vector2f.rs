use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Vector2f{
    pub x: f32,
    pub y: f32,
}

impl Vector2f{
    pub fn new(x: f32, y: f32) -> Self {
        Vector2f{
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Self {
        Vector2f{
            x: 0.0,
            y: 0.0,
        }
    }
    
    pub fn one() -> Self {
        Vector2f{
            x: 1.0,
            y: 1.0,
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
    
    pub fn magnitude_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        (x * x) + (y * y)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Vector2f::new(self.x / m, self.y / m)
    }

    pub fn scale(&self, s: f32) -> Self {
        Vector2f::new(self.x * s, self.y * s)
    }

    pub fn angle_between_rad(&self, other: Self) -> f32 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
}

impl Add for Vector2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Vector2f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl Sub for Vector2f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign for Vector2f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        };
    }
}

impl From<Vector2f> for [f32; 2] {
    fn from(vec: Vector2f) -> [f32; 2] {
        [vec.x, vec.y]
    }
}

impl PartialEq for Vector2f {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
