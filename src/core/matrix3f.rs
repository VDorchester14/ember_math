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
use bevy_reflect::{
    Reflect,
    FromReflect
};


#[derive(Debug, Clone, Serialize, Deserialize, Reflect, FromReflect)]
#[repr(C)]
pub struct Matrix3f{
    pub data: Vec<f32>,
}

impl Default for Matrix3f {
    fn default() -> Self {
        Matrix3f::identity()
    }
}

impl Matrix3f{
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32
    ) -> Self {
        Matrix3f{
            data: vec![
                m00, m01, m02,
                m10, m11, m12,
                m20, m21, m22
            ]
        }
    }

    pub fn one() -> Self{
        Matrix3f{
            data: vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ]
        }
    }

    pub fn zero() -> Self {
        Matrix3f{
            data: vec![
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0
            ]
        }
    }

    pub fn identity() -> Self{
        Matrix3f{
            data: vec![
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            ]
        }
    }

    pub fn scale(&self, s: f32) -> Self{
        Matrix3f{
            data: vec![
                self.data[0] * s, self.data[1] * s, self.data[2] * s,
                self.data[3] * s, self.data[4] * s, self.data[5] * s,
                self.data[6] * s, self.data[7] * s, self.data[8] * s
            ]
        }
    }

    pub fn transpose(&self) -> Self{
        Matrix3f{
            data: vec![
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
            m00, -m01, m02,
            -m10, m11, -m12,
            m20, -m21, m22
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

    pub fn from_axis_angle(axis_in: Vector3f, angle: f32) -> Matrix3f {
        let axis = axis_in.normalize();
        let cosr = angle.cos();
        let sinr = angle.sin();
        let omc = 1.0 - cosr;
        let r0c0 = cosr + (axis.x * axis.x * omc);
        let r0c1 = (axis.x * axis.y * omc) - (axis.z * sinr);
        let r0c2 = (axis.x * axis.z * omc) + (axis.y * sinr);
        let r1c0 = (axis.y * axis.x * omc) + (axis.z * sinr);
        let r1c1 = cosr + (axis.y * axis.y * omc);
        let r1c2 = (axis.y * axis.z * omc) - (axis.x * sinr);
        let r2c0 = (axis.z * axis.x * omc) - (axis.y * sinr);
        let r2c1 = (axis.z * axis.y * omc) + (axis.x * sinr);
        let r2c2 = cosr + (axis.z * axis.z * omc);
        Matrix3f::new(
            r0c0, r0c1, r0c2,
            r1c0, r1c1, r1c2,
            r2c0, r2c1, r2c2
        )
    }

}

impl Add for Matrix3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: vec![
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
            data: vec![
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
            data: vec![
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
            data: vec![
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
            data: vec![
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
            data: vec![
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

impl PartialEq for Matrix3f {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        return true;
    }
}