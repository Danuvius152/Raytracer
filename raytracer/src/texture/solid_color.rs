use crate::basic::vec::Vec3;

use super::Texture;

pub struct SolidColor {
    pub color_value: Vec3,
}

impl Texture for SolidColor {
    fn get_color_value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}
impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Vec3::new(r, g, b),
        }
    }
}
