use crate::basic::vec::Vec3;
pub mod checker;
pub mod perlin;
pub mod solid_color;

pub trait Texture {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
