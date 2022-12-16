pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{
    basic::{ray::Ray, vec::Vec3},
    hittable::HitRecord,
};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
}
