use crate::{random::random_double, ray::Ray, vec3::Vec3};
use rand::Rng;

fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2. * Vec3::new(rng.gen_range(0., 1.), rng.gen_range(0., 1.), 0.)
            - Vec3::new(1., 1., 0.);

        if p.dot(p) >= 1. {
            return p;
        }
    }
}

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        use std::f32::consts::PI;
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin: lookfrom,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = u * rd.x + v * rd.y;
        let origin = self.origin + offset;
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        let time = self.time0 + random_double(rng) * (self.time1 - self.time0);

        Ray::new(origin, direction, time)
    }
}
