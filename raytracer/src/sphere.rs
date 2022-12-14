#![allow(dead_code)]
use crate::{hittable::HitRecord, hittable::Hittable, ray::Ray, vec::Vec3};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3, //球心
    pub r: f64,
}
impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir * ray.dir;
        let half_b = ray.dir * oc;
        let c = oc * oc - self.r * self.r;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            Option::None
        } else {
            let sqrt = discriminant.sqrt();
            let mut root = (-half_b - sqrt) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrt) / a;
                if root < t_min || t_max < root {
                    return Option::None;
                }
            }
            let mut rec = HitRecord {
                t: root,
                p: ray.at(root),
                normal: Vec3::new(0., 0., 0.),
                front_face: true,
            };
            let outward_normal = (rec.p - self.center) / self.r;
            rec.set_face_normal(ray, outward_normal);

            Option::Some(rec)
        }
    }
}
