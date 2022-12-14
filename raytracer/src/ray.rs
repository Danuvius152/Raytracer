#![allow(dead_code)]
use crate::{hittable::Hittable, vec::Vec3};
use std::f64::INFINITY;
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
    pub fn ray_color<T>(self, world: &T) -> Vec3
    where
        T: Hittable + 'static,
    {
        if let Some(tmp_rec) = world.hit(self, 0., INFINITY) {
            (Vec3::new(1., 1., 1.) + tmp_rec.normal) * 0.5
        } else {
            let unit_direction = Vec3::unit(self.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
