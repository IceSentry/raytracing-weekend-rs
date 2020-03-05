use enum_dispatch::enum_dispatch;
use rand::Rng;

use crate::{
    hittable::HitRecord,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    random::random_double,
    ray::Ray,
    vec3::Vec3,
};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)>;
}

#[enum_dispatch(Material)]
#[derive(Clone)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random_double(rng), random_double(rng), random_double(rng))
            - Vec3::new(1., 1., 1.);
        if p.squared_norm() < 1. {
            return p;
        }
    }
}
