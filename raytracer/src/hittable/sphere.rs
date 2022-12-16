#![allow(dead_code)]
use std::rc::Rc;

use crate::{
    basic::{ray::Ray, vec::Vec3},
    hittable::{HitRecord, Hittable},
    material::Material,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3, //球心
    pub r: f64,
    pub mat_ptr: Rc<dyn Material>,
}
// pub struct Sphere<T>
// where
//     T: Material,
// {
//     pub center: Vec3,
//     pub radius: f64,
//     pub mat: T, //不保存指针，直接保存结构体
// }

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
                mat_ptr: self.mat_ptr.clone(),
            };
            let outward_normal = (rec.p - self.center) / self.r;
            rec.set_face_normal(ray, outward_normal);

            Option::Some(rec)
        }
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub r: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - MovingSphere::center(&self, ray.time);
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
                mat_ptr: self.mat_ptr.clone(),
            };
            let outward_normal = (rec.p - MovingSphere::center(&self, ray.time)) / self.r;
            rec.set_face_normal(ray, outward_normal);

            Option::Some(rec)
        }
    }
}
