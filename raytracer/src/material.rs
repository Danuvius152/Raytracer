#![allow(dead_code)]

use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_sphere();
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit(r_in.dir), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if scattered.dir * rec.normal > 0. {
            Some(ScatterRecord {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
