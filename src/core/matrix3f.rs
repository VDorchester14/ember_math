use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign
};

use crate::core::vector3f::Vector3f;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Matrix3f{
    pub data: [f32; 9] // row major
}

impl Matrix3f{
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32
    ) -> Self {
        Matrix3f{
            data: [
                m00, m01, m02,
                m10, m11, m12,
                m20, m21, m22
            ]
        }
    }

    pub fn one() -> Self{
        Matrix3f{
            data: [
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ]
        }
    }

    pub fn identity() -> Self{
        Matrix3f{
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            ]
        }
    }

    pub fn scale(&self, s: f32) -> Self{
        Matrix3f{
            data: [
                self.data[0] * s, self.data[1] * s, self.data[2] * s,
                self.data[3] * s, self.data[4] * s, self.data[5] * s,
                self.data[6] * s, self.data[7] * s, self.data[8] * s
            ]
        }
    }

    pub fn transpose(&self) -> Self{
        Matrix3f{
            data:[
                self.data[0], self.data[3], self.data[6],
                self.data[1], self.data[4], self.data[7],
                self.data[2], self.data[5], self.data[8]
            ]
        }
    }

    pub fn transform(&self, other: Vector3f) -> Vector3f {
        Vector3f{
            x: self.data[0]*other.x + self.data[1]*other.y + self.data[2]*other.z,
            y: self.data[3]*other.x + self.data[4]*other.y + self.data[5]*other.z,
            z: self.data[6]*other.x + self.data[7]*other.y + self.data[8]*other.z
        }
    }

    pub fn determinant(&self) -> f32 {
        let a = self.data[0] * (self.data[4]*self.data[8] - self.data[5]*self.data[7]);
        let b = self.data[1] * (self.data[3]*self.data[8] - self.data[5]*self.data[6]);
        let c = self.data[2] * (self.data[3]*self.data[7] - self.data[4]*self.data[6]);
        a - b + c
    }

    pub fn cofactor(&self) -> Self {
        let m00 = self.data[4]*self.data[8] - self.data[5]*self.data[7];
        let m01 = self.data[3]*self.data[8] - self.data[5]*self.data[6];
        let m02 = self.data[3]*self.data[7] - self.data[4]*self.data[6];

        let m10 = self.data[1]*self.data[8] - self.data[2]*self.data[7];
        let m11 = self.data[0]*self.data[8] - self.data[2]*self.data[6];
        let m12 = self.data[0]*self.data[7] - self.data[1]*self.data[6];

        let m20 = self.data[1]*self.data[5] - self.data[2]*self.data[4];
        let m21 = self.data[0]*self.data[5] - self.data[2]*self.data[3];
        let m22 = self.data[0]*self.data[4] - self.data[1]*self.data[3];
        
        Matrix3f::new(
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22
        )
    }

    pub fn adjugate(&self) -> Self {
        self.cofactor().transpose()
    }

    pub fn inverse(&self) -> Self {
        self.adjugate().scale(1.0 / self.determinant())
    }

    pub fn invertible(&self) -> bool {
        self.determinant() > 1e-6
    }
}

impl Add for Matrix3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2],
                self.data[3] + other.data[3], self.data[4] + other.data[4], self.data[5] + other.data[5],
                self.data[6] + other.data[6], self.data[7] + other.data[7], self.data[8] + other.data[8]
            ]
        }
    }
}

impl AddAssign for Matrix3f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            data: [
                self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2],
                self.data[3] + other.data[3], self.data[4] + other.data[4], self.data[5] + other.data[5],
                self.data[6] + other.data[6], self.data[7] + other.data[7], self.data[8] + other.data[8]
            ]
        };
    }
}

impl Sub for Matrix3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2],
                self.data[3] - other.data[3], self.data[4] - other.data[4], self.data[5] - other.data[5],
                self.data[6] - other.data[6], self.data[7] - other.data[7], self.data[8] - other.data[8]
            ]
        }
    }
}

impl SubAssign for Matrix3f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            data: [
                self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2],
                self.data[3] - other.data[3], self.data[4] - other.data[4], self.data[5] - other.data[5],
                self.data[6] - other.data[6], self.data[7] - other.data[7], self.data[8] - other.data[8]
            ]
        };
    }
}

impl Mul for Matrix3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data:[
                (self.data[0]*other.data[0]) + (self.data[1]*other.data[3]) + (self.data[2]*other.data[6]),
                (self.data[0]*other.data[1]) + (self.data[1]*other.data[4]) + (self.data[2]*other.data[7]),
                (self.data[0]*other.data[2]) + (self.data[1]*other.data[5]) + (self.data[2]*other.data[8]),
                (self.data[3]*other.data[0]) + (self.data[4]*other.data[3]) + (self.data[5]*other.data[6]),
                (self.data[3]*other.data[1]) + (self.data[4]*other.data[4]) + (self.data[5]*other.data[7]),
                (self.data[3]*other.data[2]) + (self.data[4]*other.data[5]) + (self.data[5]*other.data[8]),
                (self.data[6]*other.data[0]) + (self.data[7]*other.data[3]) + (self.data[8]*other.data[6]),
                (self.data[6]*other.data[1]) + (self.data[7]*other.data[4]) + (self.data[8]*other.data[7]),
                (self.data[6]*other.data[2]) + (self.data[7]*other.data[5]) + (self.data[8]*other.data[8])
            ]
        }
    }
}

impl MulAssign for Matrix3f {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            data:[
                (self.data[0]*other.data[0]) + (self.data[1]*other.data[3]) + (self.data[2]*other.data[6]),
                (self.data[0]*other.data[1]) + (self.data[1]*other.data[4]) + (self.data[2]*other.data[7]),
                (self.data[0]*other.data[2]) + (self.data[1]*other.data[5]) + (self.data[2]*other.data[8]),
                (self.data[3]*other.data[0]) + (self.data[4]*other.data[3]) + (self.data[5]*other.data[6]),
                (self.data[3]*other.data[1]) + (self.data[4]*other.data[4]) + (self.data[5]*other.data[7]),
                (self.data[3]*other.data[2]) + (self.data[4]*other.data[5]) + (self.data[5]*other.data[8]),
                (self.data[6]*other.data[0]) + (self.data[7]*other.data[3]) + (self.data[8]*other.data[6]),
                (self.data[6]*other.data[1]) + (self.data[7]*other.data[4]) + (self.data[8]*other.data[7]),
                (self.data[6]*other.data[2]) + (self.data[7]*other.data[5]) + (self.data[8]*other.data[8])
            ]
        };
    }
}

impl From<Matrix3f> for [[f32; 3]; 3] {
    fn from(m: Matrix3f) -> [[f32; 3]; 3] {
        [
            [m.data[0], m.data[1], m.data[2]],
            [m.data[3], m.data[4], m.data[5]],
            [m.data[6], m.data[7], m.data[8]],
        ]
    }
}