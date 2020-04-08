use crate::{random::random_double, vec3::Vec3};
use rand::Rng;

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random_double(rng), random_double(rng), random_double(rng))
            - Vec3::new(1., 1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}
