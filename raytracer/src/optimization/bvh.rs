#![allow(dead_code)]
use crate::{
    basic::ray::Ray,
    hittable::{HitRecord, Hittable, HittableList},
    optimization::aabb::AABB,
    utility,
};
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(Clone)]
pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub ab_box: AABB, // box 是原有的关键字
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.ab_box.hit(r, t_min, t_max) {
            return None;
        }
        let mut hit_rec = None;
        let mut closest_so_far = t_max;
        if let Some(hitleft) = self.left.hit(r, t_min, t_max) {
            closest_so_far = hitleft.t;
            hit_rec = Some(hitleft);
        }
        if let Some(hitright) = self.right.hit(r, t_min, closest_so_far) {
            hit_rec = Some(hitright);
        }
        hit_rec
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.ab_box)
    }
}

impl BvhNode {
    pub fn box_cmp(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let mut box_a: AABB = Default::default();
        let mut box_b: AABB = Default::default();
        let mut flag1 = false;
        let mut flag2 = false;
        if let Some(_box_a) = a.bounding_box(0., 0.) {
            box_a = _box_a;
            flag1 = true;
        }
        if let Some(_box_b) = b.bounding_box(0., 0.) {
            box_b = _box_b;
            flag2 = true;
        }
        if !flag1 || !flag2 {
            panic!("No bounding box in BvhNode constructor!");
        }
        box_a.min[axis].partial_cmp(&box_b.min[axis]).expect("NaN")
    }

    //三个特例
    pub fn x_cmp(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_cmp(a, b, 0)
    }
    pub fn y_cmp(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_cmp(a, b, 1)
    }
    pub fn z_cmp(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_cmp(a, b, 2)
    }

    pub fn new_from_vec(src_objects: &[Rc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let x = src_objects.len(); // &Vec<Rc<dyn Hittable>>
        BvhNode::new_with_5para(src_objects, 0, x, time0, time1)
    }

    pub fn new_from_list(list: HittableList, time0: f64, time1: f64) -> Self {
        BvhNode::new_from_vec(&list.objects, time0, time1)
    }

    pub fn new_with_5para(
        src_objects: &[Rc<dyn Hittable>], // &Vec<Rc<dyn Hittable>>
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects[start..end].to_vec();
        let axis = utility::random_int(0, 2);
        let comparator = if axis == 0 {
            BvhNode::x_cmp
        } else if axis == 1 {
            BvhNode::y_cmp
        } else {
            BvhNode::z_cmp
        };
        let object_span = end - start;
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        let mut box_a: AABB = Default::default();
        let mut box_b: AABB = Default::default();
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::new_with_5para(
                src_objects,
                start,
                mid,
                time0,
                time1,
            ));
            right = Rc::new(BvhNode::new_with_5para(src_objects, mid, end, time0, time1));
            let mut flag1 = false;
            let mut flag2 = false;
            if let Some(_box_a) = left.bounding_box(time0, time1) {
                box_a = _box_a;
                flag1 = true;
            }
            if let Some(_box_b) = right.bounding_box(time0, time1) {
                box_b = _box_b;
                flag2 = true;
            }
            if !flag1 || !flag2 {
                panic!("No bounding box in BvhNode constructor!");
            }
        }
        Self {
            left,
            right,
            ab_box: AABB::surrounding_box(box_a, box_b),
        }
    }
}
