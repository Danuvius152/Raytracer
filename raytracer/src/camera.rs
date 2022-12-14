use crate::{ray::Ray, vec::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}
impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);
        Camera {
            origin: (origin),
            lower_left_corner: (lower_left_corner),
            horizontal: (horizontal),
            vertical: (vertical),
        }
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray {
            dir: (self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
            orig: (self.origin),
        }
    }
}
