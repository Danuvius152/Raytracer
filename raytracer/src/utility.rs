#![allow(dead_code)]
use rand::Rng;
use std::f64::consts::PI;

use crate::basic::vec::Vec3;

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + rng.gen::<f64>() * (max - min)
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a <= b {
        b
    } else {
        a
    }
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
pub fn get_pixel_color(color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    let scale = 1. / samples_per_pixel as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();
    [
        (clamp(r, 0., 0.999) * 256.).floor() as u8,
        (clamp(g, 0., 0.999) * 256.).floor() as u8,
        (clamp(b, 0., 0.999) * 256.).floor() as u8,
    ]
}
