use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign
};
use std::f32::consts::PI;

use crate::core::vector4f::Vector4f;
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
pub struct Matrix4f{
    pub data: Vec<f32>,
}

impl Default for Matrix4f {
    fn default() -> Self {
        Matrix4f::identity()
    }
}

impl Matrix4f{
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32
    ) -> Self {
        Matrix4f{
            data: vec![
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33
            ]
        }
    }

    pub fn one() -> Self{
        Matrix4f{
            data: vec![
                1.0, 1.0, 1.0, 1.0, 
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0
            ]
        }
    }

    pub fn identity() -> Self{
        Matrix4f{
            data: vec![
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn scale(&self, s: f32) -> Self{
        Matrix4f{
            data: vec![
                self.data[0] * s, self.data[1] * s, self.data[2] * s, self.data[3] * s,
                self.data[4] * s, self.data[5] * s, self.data[6] * s, self.data[7] * s,
                self.data[8] * s, self.data[9] * s, self.data[10] * s, self.data[11] * s,
                self.data[12] * s, self.data[13] * s, self.data[14] * s, self.data[15] * s
            ]
        }
    }

    pub fn transpose(&self) -> Self{
        Matrix4f{
            data: vec![
                self.data[0], self.data[4], self.data[8], self.data[12],
                self.data[1], self.data[5], self.data[9], self.data[13],
                self.data[2], self.data[6], self.data[10], self.data[14],
                self.data[3], self.data[7], self.data[11], self.data[15]
            ]
        }
    }

    pub fn transform(&self, other: Vector4f) -> Vector4f {
        Vector4f{
            x: self.data[0]*other.x + self.data[1]*other.y + self.data[2]*other.z + self.data[3]*other.w,
            y: self.data[4]*other.x + self.data[5]*other.y + self.data[6]*other.z + self.data[7]*other.w,
            z: self.data[8]*other.x + self.data[9]*other.y + self.data[10]*other.z + self.data[11]*other.w,
            w: self.data[12]*other.x + self.data[13]*other.y + self.data[14]*other.z + self.data[15]*other.w
        }
    }

    pub fn from_translation(t: Vector3f) -> Matrix4f {
        Matrix4f{
            data: vec![
                1.0, 0.0, 0.0, t.x,
                0.0, 1.0, 0.0, t.y,
                0.0, 0.0, 1.0, t.z,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn from_scale(s: f32) -> Matrix4f {
        Matrix4f{
            data: vec![
                s, 0.0, 0.0, 0.0,
                0.0, s, 0.0, 0.0,
                0.0, 0.0, s, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn from_scale_vec(s: Vector3f) -> Matrix4f {
        Matrix4f{
            data: vec![
                s.x, 0.0, 0.0, 0.0,
                0.0, s.y, 0.0, 0.0,
                0.0, 0.0, s.z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn from_angle_x_deg(r: f32) -> Matrix4f {
        let b = r * (PI / 180.0);
        Matrix4f::from_angle_x(b)
    }

    pub fn from_angle_x(r: f32) -> Matrix4f {
        Matrix4f{
            data: vec![
                1.0, 0.0, 0.0, 0.0,
                0.0, r.cos(), -r.sin(), 0.0,
                0.0, r.sin(), r.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn from_angle_y_deg(r: f32) -> Matrix4f {
        let b = r * (PI / 180.0);
        Matrix4f::from_angle_y(b)
    }

    pub fn from_angle_y(r: f32) -> Matrix4f {
        Matrix4f{
            data: vec![
                r.cos(), 0.0, r.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -r.sin(), 0.0, r.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn from_angle_z_deg(r: f32) -> Matrix4f {
        let b = r * (PI / 180.0);
        Matrix4f::from_angle_z(b)
    }

    pub fn from_angle_z(r: f32) -> Matrix4f {
        Matrix4f{
            data: vec![
                r.cos(), -r.sin(), 0.0, 0.0,
                r.sin(), r.cos(), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn from_axis_angle(axis: Vector3f, r: f32) -> Matrix4f {
        Matrix4f::from_axis_angle_comp(axis.x, axis.y, axis.z, r)
    }

    pub fn from_axis_angle_comp(x: f32, y: f32, z: f32, r: f32) -> Matrix4f {
        let cosr = r.cos();
        let sinr = r.sin();
        let a = 1.0 - cosr;
        let xsin = x * sinr;
        let ysin = y * sinr;
        let zsin = y * sinr;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        Matrix4f{
            data: vec![
                cosr + xx*a, xy*a - z*sinr, xz*a + ysin, 0.0,
                xy*a + zsin, cosr + yy*a, yz*a - xsin, 0.0, 
                xz*a - ysin, yz*a + xsin, cosr + zz*a, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn look_at_rh(eye: Vector3f, center: Vector3f, up: Vector3f) -> Matrix4f {
        let dir = center - eye;
        let zaxis = dir.normalize();
        let xaxis = zaxis.cross(up).normalize();
        let yaxis = xaxis.cross(zaxis).normalize();

        Matrix4f::new(
            xaxis.x, xaxis.y, xaxis.z, -eye.dot(xaxis),
            yaxis.x, yaxis.y, yaxis.z, -eye.dot(yaxis), 
            -zaxis.x, -zaxis.y, -zaxis.z, eye.dot(zaxis),
            0.0, 0.0, 0.0, 1.0
        ).transpose()
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix4f {
        let c = 1.0 / (fovy / 2.0).tan();
        let fp = far;
        let np = near;

        let c0r0 = c / aspect;
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = c;
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 = 0.0;
        let c2r1 = 0.0;
        // let c2r2 = -(fp + np) / (fp - np);
        let c2r2 = (fp + np) / (np - fp);  // from cgmath
        let c2r3 = -1.0;

        let c3r0 = 0.0;
        let c3r1 = 0.0;
        // let c3r2 = -(2.0 * fp * np) / (fp - np);
        let c3r2 = (2.0 * fp * np) / (np - fp);  // from cgmath
        let c3r3 = 0.0;

        Matrix4f::new(
            c0r0, c1r0, c2r0, c3r0,
            c0r1, c1r1, c2r1, c3r1,
            c0r2, c1r2, c2r2, c3r2,
            c0r3, c1r3, c2r3, c3r3,
        ).transpose()
    }
}

impl Add for Matrix4f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: vec![
                self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2], self.data[3] + other.data[3],
                self.data[4] + other.data[4], self.data[5] + other.data[5], self.data[6] + other.data[6], self.data[7] + other.data[7],
                self.data[8] + other.data[8], self.data[9] + other.data[9], self.data[10] + other.data[10], self.data[11] + other.data[11],
                self.data[12] + other.data[12], self.data[13] + other.data[13], self.data[14] + other.data[14], self.data[15] + other.data[15]
            ]
        }
    }
}

impl AddAssign for Matrix4f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            data: vec![
                self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2], self.data[3] + other.data[3],
                self.data[4] + other.data[4], self.data[5] + other.data[5], self.data[6] + other.data[6], self.data[7] + other.data[7],
                self.data[8] + other.data[8], self.data[9] + other.data[9], self.data[10] + other.data[10], self.data[11] + other.data[11],
                self.data[12] + other.data[12], self.data[13] + other.data[13], self.data[14] + other.data[14], self.data[15] + other.data[15]
            ]
        };
    }
}

impl Sub for Matrix4f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: vec![
                self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2], self.data[3] - other.data[3],
                self.data[4] - other.data[4], self.data[5] - other.data[5], self.data[6] - other.data[6], self.data[7] - other.data[7],
                self.data[8] - other.data[8], self.data[9] - other.data[9], self.data[10] - other.data[10], self.data[11] - other.data[11],
                self.data[12] - other.data[12], self.data[13] - other.data[13], self.data[14] - other.data[14], self.data[15] - other.data[15]
            ]
        }
    }
}

impl SubAssign for Matrix4f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            data: vec![
                self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2], self.data[3] - other.data[3],
                self.data[4] - other.data[4], self.data[5] - other.data[5], self.data[6] - other.data[6], self.data[7] - other.data[7],
                self.data[8] - other.data[8], self.data[9] - other.data[9], self.data[10] - other.data[10], self.data[11] - other.data[11],
                self.data[12] - other.data[12], self.data[13] - other.data[13], self.data[14] - other.data[14], self.data[15] - other.data[15]
            ]
        };
    }
}

impl Mul for Matrix4f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: vec![
                (self.data[0]*other.data[0]) + (self.data[1]*other.data[4]) + (self.data[2]*other.data[8]) + (self.data[3]*self.data[12]),
                (self.data[0]*other.data[1]) + (self.data[1]*other.data[5]) + (self.data[2]*other.data[9]) + (self.data[3]*self.data[13]),
                (self.data[0]*other.data[2]) + (self.data[1]*other.data[6]) + (self.data[2]*other.data[10]) + (self.data[3]*self.data[14]),
                (self.data[0]*other.data[3]) + (self.data[1]*other.data[7]) + (self.data[2]*other.data[11]) + (self.data[3]*self.data[15]),

                (self.data[4]*other.data[0]) + (self.data[5]*other.data[4]) + (self.data[6]*other.data[8]) + (self.data[7]*self.data[12]),
                (self.data[4]*other.data[1]) + (self.data[5]*other.data[5]) + (self.data[6]*other.data[9]) + (self.data[7]*self.data[13]),
                (self.data[4]*other.data[2]) + (self.data[5]*other.data[6]) + (self.data[6]*other.data[10]) + (self.data[7]*self.data[14]),
                (self.data[4]*other.data[3]) + (self.data[5]*other.data[7]) + (self.data[6]*other.data[11]) + (self.data[7]*self.data[15]),

                (self.data[8]*other.data[0]) + (self.data[9]*other.data[4]) + (self.data[10]*other.data[8]) + (self.data[11]*self.data[12]),
                (self.data[8]*other.data[1]) + (self.data[9]*other.data[5]) + (self.data[10]*other.data[9]) + (self.data[11]*self.data[13]),
                (self.data[8]*other.data[2]) + (self.data[9]*other.data[6]) + (self.data[10]*other.data[10]) + (self.data[11]*self.data[14]),
                (self.data[8]*other.data[3]) + (self.data[9]*other.data[7]) + (self.data[10]*other.data[11]) + (self.data[11]*self.data[15]),

                (self.data[12]*other.data[0]) + (self.data[13]*other.data[4]) + (self.data[14]*other.data[8]) + (self.data[15]*self.data[12]),
                (self.data[12]*other.data[1]) + (self.data[13]*other.data[5]) + (self.data[14]*other.data[9]) + (self.data[15]*self.data[13]),
                (self.data[12]*other.data[2]) + (self.data[13]*other.data[6]) + (self.data[14]*other.data[10]) + (self.data[15]*self.data[14]),
                (self.data[12]*other.data[3]) + (self.data[13]*other.data[7]) + (self.data[14]*other.data[11]) + (self.data[15]*self.data[15])
            ]
        }
    }
}

impl MulAssign for Matrix4f {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            data: vec![
                (self.data[0]*other.data[0]) + (self.data[1]*other.data[4]) + (self.data[2]*other.data[8]) + (self.data[3]*self.data[12]),
                (self.data[0]*other.data[1]) + (self.data[1]*other.data[5]) + (self.data[2]*other.data[9]) + (self.data[3]*self.data[13]),
                (self.data[0]*other.data[2]) + (self.data[1]*other.data[6]) + (self.data[2]*other.data[10]) + (self.data[3]*self.data[14]),
                (self.data[0]*other.data[3]) + (self.data[1]*other.data[7]) + (self.data[2]*other.data[11]) + (self.data[3]*self.data[15]),

                (self.data[4]*other.data[0]) + (self.data[5]*other.data[4]) + (self.data[6]*other.data[8]) + (self.data[7]*self.data[12]),
                (self.data[4]*other.data[1]) + (self.data[5]*other.data[5]) + (self.data[6]*other.data[9]) + (self.data[7]*self.data[13]),
                (self.data[4]*other.data[2]) + (self.data[5]*other.data[6]) + (self.data[6]*other.data[10]) + (self.data[7]*self.data[14]),
                (self.data[4]*other.data[3]) + (self.data[5]*other.data[7]) + (self.data[6]*other.data[11]) + (self.data[7]*self.data[15]),

                (self.data[8]*other.data[0]) + (self.data[9]*other.data[4]) + (self.data[10]*other.data[8]) + (self.data[11]*self.data[12]),
                (self.data[8]*other.data[1]) + (self.data[9]*other.data[5]) + (self.data[10]*other.data[9]) + (self.data[11]*self.data[13]),
                (self.data[8]*other.data[2]) + (self.data[9]*other.data[6]) + (self.data[10]*other.data[10]) + (self.data[11]*self.data[14]),
                (self.data[8]*other.data[3]) + (self.data[9]*other.data[7]) + (self.data[10]*other.data[11]) + (self.data[11]*self.data[15]),

                (self.data[12]*other.data[0]) + (self.data[13]*other.data[4]) + (self.data[14]*other.data[8]) + (self.data[15]*self.data[12]),
                (self.data[12]*other.data[1]) + (self.data[13]*other.data[5]) + (self.data[14]*other.data[9]) + (self.data[15]*self.data[13]),
                (self.data[12]*other.data[2]) + (self.data[13]*other.data[6]) + (self.data[14]*other.data[10]) + (self.data[15]*self.data[14]),
                (self.data[12]*other.data[3]) + (self.data[13]*other.data[7]) + (self.data[14]*other.data[11]) + (self.data[15]*self.data[15])
            ]
        };
    }
}

impl From<Matrix4f> for [[f32; 4]; 4] {
    fn from(m: Matrix4f) -> [[f32; 4]; 4] {
        [
            [m.data[0], m.data[1], m.data[2], m.data[3]],
            [m.data[4], m.data[5], m.data[6], m.data[7]],
            [m.data[8], m.data[9], m.data[10], m.data[11]],
            [m.data[12], m.data[13], m.data[14], m.data[15]]
        ]
    }
}

impl PartialEq for Matrix4f {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        return true;
    }
}