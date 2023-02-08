pub mod sphere;

use std::rc::Rc;

use crate::basic::{ray::Ray, vec::Vec3};
use crate::material::Material;
use crate::optimization::aabb::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,      //碰撞点
    pub normal: Vec3, //法向量
    pub t: f64,
    pub front_face: bool,                   //方向是否为外侧
    pub mat_ptr: std::rc::Rc<dyn Material>, //材料
    pub u: f64,
    pub v: f64,
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    //判断光线在 [t_min, t_max] 内是否碰到物体
    //优化，用 Option 是否为 None 来判断碰撞与否，同时包括返回值
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB>;
    // AABB 优化，判断光线是否撞到 大的 box
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
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
        self.objects.push(Rc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest_so_far = t_max;
        for i in &self.objects {
            if let Some(temp_rec) = i.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_rec = Some(temp_rec);
            }
        }
        hit_rec
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {
            let mut output_box = AABB {
                //不能Option
                min: Vec3::new(0., 0., 0.),
                max: Vec3::new(0., 0., 0.),
            };
            let mut is_first_box = true;
            for i in &self.objects {
                if let Some(temp_box) = i.bounding_box(_t0, _t1) {
                    if is_first_box {
                        output_box = temp_box;
                    } else {
                        output_box = AABB::surrounding_box(output_box, temp_box);
                    }
                } else {
                    return None;
                }
                is_first_box = false;
            }
            Some(output_box)
        }
    }
}
