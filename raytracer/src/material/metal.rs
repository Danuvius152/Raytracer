use super::{Material, ScatterRecord};
use crate::{
    basic::{ray::Ray, vec::Vec3},
    hittable::HitRecord,
};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit(r_in.dir), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time,
        );
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
