use crate::{basic::vec::Vec3, hittable::Hittable};
use std::f64::INFINITY;
#[derive(Copy, Clone)]
pub struct Ray {
    pub dir: Vec3,  //方向
    pub orig: Vec3, //原点
    pub time: f64,  //时间
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3, time: f64) -> Self {
        Self { dir, orig, time }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn ray_color<T>(self, world: &T, depth: i32) -> Vec3
    where
        T: Hittable + 'static,
    {
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }
        if let Some(tmp_rec) = world.hit(self, 0.001, INFINITY) {
            if let Some(tmp_scatter) = tmp_rec.mat_ptr.scatter(self, tmp_rec.clone()) {
                Vec3::elemul(
                    tmp_scatter.attenuation,
                    Ray::ray_color(tmp_scatter.scattered, world, depth - 1),
                )
            } else {
                Vec3::new(0., 0., 0.)
            }

            // (Vec3::new(1., 1., 1.) + tmp_rec.normal) * 0.5
            //let target = tmp_rec.p + tmp_rec.normal + Vec3::random_unit_sphere();
            //let target = tmp_rec.p + Vec3::random_in_hemisphere(tmp_rec.normal);
            //Ray::ray_color(Ray::new(tmp_rec.p, target - tmp_rec.p), world, depth - 1) * 0.5
        } else {
            let unit_direction = Vec3::unit(self.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
