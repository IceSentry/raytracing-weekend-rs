use crate::{hittable::HitRecord, random::random_double, ray::Ray, vec3::Vec3};
use rand::rngs::ThreadRng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f32 },
    Dielectric { ref_idx: f32 },
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

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random_double(rng), random_double(rng), random_double(rng))
            - Vec3::new(1., 1., 1.);
        if p.squared_norm() < 1. {
            return p;
        }
    }
}

pub fn scatter(r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
    match rec.mat {
        Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere(rng);
            Some((Ray::new(rec.p, target - rec.p), albedo))
        }
        Material::Metal { albedo, fuzziness } => {
            let fuzz = if fuzziness < 1. { fuzziness } else { 1. };

            let reflected = reflect(r_in.direction.unit(), rec.normal);
            let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere(rng));
            if scattered.direction.dot(rec.normal) > 0. {
                return Some((scattered, albedo));
            }
            None
        }
        Material::Dielectric { ref_idx } => {
            let outward_normal: Vec3;
            let reflected = reflect(r_in.direction, rec.normal);
            let ni_over_nt: f32;
            let attenuation = Vec3::new(1., 1., 1.);
            let cosine: f32;

            if r_in.direction.dot(rec.normal) > 0. {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.norm()
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1. / ref_idx;
                cosine = -r_in.direction.dot(rec.normal) / r_in.direction.norm()
            }

            let scattered = match refract(r_in.direction, outward_normal, ni_over_nt) {
                Some(refracted) => {
                    if random_double(rng) > schlick(cosine, ref_idx) {
                        refracted
                    } else {
                        reflected
                    }
                }
                None => reflected,
            };

            Some((Ray::new(rec.p, scattered), attenuation))
        }
    }
}
