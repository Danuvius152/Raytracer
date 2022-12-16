use super::{Material, ScatterRecord};
use crate::{
    basic::{ray::Ray, vec::Vec3},
    hittable::HitRecord,
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_sphere();
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, scatter_direction, r_in.time),
            attenuation: self.albedo,
        })
    }
}
