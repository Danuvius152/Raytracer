use crate::{
    basic::{ray::Ray, vec::Vec3},
    utility,
};
use std::mem::swap;

#[derive(Copy, Clone, Default)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }
    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1. / r.dir[i];
            let mut t0 = (self.min[i] - r.orig[i]) * inv_d;
            let mut t1 = (self.max[i] - r.orig[i]) * inv_d;
            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            t_min = utility::fmax(t0, t_min);
            t_max = utility::fmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            utility::fmin(box0.min[0], box1.min[0]),
            utility::fmin(box0.min[1], box1.min[1]),
            utility::fmin(box0.min[2], box1.min[2]),
        );

        let big = Vec3::new(
            utility::fmax(box0.max[0], box1.max[0]),
            utility::fmax(box0.max[1], box1.max[1]),
            utility::fmax(box0.max[2], box1.max[2]),
        );
        AABB::new(small, big)
    }
}
