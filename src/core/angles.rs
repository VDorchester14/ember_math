use std::f32::consts::PI;

pub fn Deg(rad: f32) -> f32 {
    let RTD = 180.0 / PI;
    rad * RTD
}

pub fn Rad(deg: f32) -> f32 {
    let DTR = PI / 180.0;
    deg * DTR
}