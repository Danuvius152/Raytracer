use super::Texture;
use crate::basic::vec::Vec3;
use std::rc::Rc;
pub struct Checker {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl Texture for Checker {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.get_color_value(u, v, p)
        } else {
            self.even.get_color_value(u, v, p)
        }
    }
}
