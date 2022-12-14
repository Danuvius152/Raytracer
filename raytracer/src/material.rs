#![allow(dead_code)]

use crate::{hittable::HitRecord, ray::Ray, utility, vec::Vec3};

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

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        //利用 Schlick's approximation 进行估计
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio;
        if rec.front_face {
            refraction_ratio = 1. / self.ref_idx;
        } else {
            refraction_ratio = self.ref_idx;
        }
        let unit_direction = Vec3::unit(r_in.dir);
        let cos = utility::fmin(-unit_direction * rec.normal, 1.);
        let sin = (1. - cos * cos).sqrt();
        let cannot_refract = refraction_ratio * sin > 1.;
        let direction;
        if cannot_refract
            || Dielectric::reflectance(cos, refraction_ratio) > utility::random_double(0., 1.)
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, direction),
            attenuation: Vec3::new(1., 1., 1.),
        })
    }
}
