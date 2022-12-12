#![allow(dead_code)]
use crate::vec::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub dir: Vec3,  //方向
    pub orig: Vec3, //原点
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { dir, orig }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn hit_sphere(center: Vec3, r: f64, ray: Ray) -> f64 {
        let oc = ray.orig - center;
        let a = ray.dir * ray.dir;
        let half_b = ray.dir * oc;
        let c = oc * oc - r * r;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            -1.
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }

    pub fn ray_color(self) -> Vec3 {
        let t = Ray::hit_sphere(Vec3::new(0., 0., -1.), 0.5, self);
        if t > 0. {
            let n = Vec3::unit(self.at(t) - Vec3::new(0., 0., -1.)); //求出法向量
            Vec3::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5
        } else {
            let unit_direction = Vec3::unit(self.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
