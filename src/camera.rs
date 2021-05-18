use crate::{ray::Ray, vec3::Vec3};
use derive_builder::*;
use rand::Rng;
use std::ops::Range;

fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2. * Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.)
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
    pub exposure: Range<f32>,
    pub height: u32,
    pub width: u32,
}

#[derive(Default, Builder)]
pub struct CameraConfig {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vfov: f32,
    pub focus_dist: f32,
    pub height: u32,
    pub width: u32,
    #[builder(default = "Vec3::new(0., 1., 0.)")]
    pub vup: Vec3,
    #[builder(default = "self.default_aspect()")]
    pub aspect: f32,
    #[builder(default = "0.0")]
    pub aperture: f32,
    #[builder(default = "Exposure(0.0..1.0)")]
    pub exposure: Exposure,
}

#[derive(Clone)]
pub struct Exposure(Range<f32>);

impl Default for Exposure {
    fn default() -> Self {
        Self(0.0..1.0)
    }
}

impl CameraConfigBuilder {
    fn default_aspect(&self) -> f32 {
        self.width.unwrap() as f32 / self.height.unwrap() as f32
    }
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(config: CameraConfig) -> Self {
        use std::f32::consts::PI;
        let theta = config.vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = config.aspect * half_height;

        let w = (config.lookfrom - config.lookat).normalize();
        let u = config.vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: config.lookfrom
                - half_width * config.focus_dist * u
                - half_height * config.focus_dist * v
                - config.focus_dist * w,
            horizontal: 2. * half_width * config.focus_dist * u,
            vertical: 2. * half_height * config.focus_dist * v,
            origin: config.lookfrom,
            u,
            v,
            w,
            lens_radius: config.aperture / 2.,
            exposure: config.exposure.0,
            width: config.width,
            height: config.height,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = u * rd.x + v * rd.y;
        let offset = Vec3::splat(offset);
        let origin = self.origin + offset;
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        let time = rng.gen_range(self.exposure.start..self.exposure.end);

        Ray::new(origin, direction, time)
    }
}
