#![allow(dead_code)]
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3,      //碰撞点
    pub normal: Vec3, //法向量
    pub t: f64,
    pub front_face: bool, //方向是否为外侧
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.dir * outward_normal < 0.;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
pub trait Hittable {
    //特性，用于实现继承
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    //优化，用 Option 是否为 None 来判断碰撞与否，同时包括返回值
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    //pub fn add(&mut self, object:Box<dyn Hittable>)
    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = Option::None;
        let mut closest_so_far = t_max;
        for i in &self.objects {
            if let Some(temp_rec) = i.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_rec = Some(temp_rec);
            }
        }
        hit_rec
    }
}
