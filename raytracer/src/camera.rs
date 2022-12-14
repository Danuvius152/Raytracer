use crate::{ray::Ray, utility, vec::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64, // top to bottom, in degrees
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = utility::degree_to_radian(vfov);
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::unit(Vec3::cross(w, u));
        let origin = lookfrom;
        Camera {
            origin,
            lower_left_corner: origin - (u * half_width + v * half_height + w) * focus_dist,
            horizontal: u * 2. * half_width * focus_dist,
            vertical: v * 2. * half_height * focus_dist,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            dir: (self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset),
            orig: (self.origin + offset),
        }
    }
}
