use std::f32::consts::PI;

pub fn deg(rad: f32) -> f32 {
    let rtd = 180.0 / PI;
    rad * rtd
}

pub fn rad(deg: f32) -> f32 {
    let dtr = PI / 180.0;
    deg * dtr
}