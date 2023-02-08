use std::rc::Rc;

use super::{Material, ScatterRecord};
use crate::{
    basic::{ray::Ray, vec::Vec3},
    hittable::HitRecord,
    texture::Texture,
};

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_sphere();
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, scatter_direction, r_in.time),
            attenuation: self.albedo.get_color_value(rec.u, rec.v, rec.p),
        })
    }
}
