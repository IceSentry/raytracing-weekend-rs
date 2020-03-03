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

pub fn scatter(ray_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
    match rec.mat {
        Material::Lambertian { albedo } => {
            let target = rec.point + rec.normal + random_in_unit_sphere(rng);
            Some((Ray::new(rec.point, target - rec.point, 0.), albedo))
        }
        Material::Metal { albedo, fuzziness } => {
            let fuzz = if fuzziness < 1. { fuzziness } else { 1. };

            let reflected = reflect(ray_in.direction.unit(), rec.normal);
            let scattered = Ray::new(
                rec.point,
                reflected + fuzz * random_in_unit_sphere(rng),
                ray_in.time,
            );
            if scattered.direction.dot(rec.normal) > 0. {
                return Some((scattered, albedo));
            }
            None
        }
        Material::Dielectric { ref_idx } => {
            let outward_normal: Vec3;
            let reflected = reflect(ray_in.direction, rec.normal);
            let ni_over_nt: f32;
            let attenuation = Vec3::new(1., 1., 1.);
            let cosine: f32;

            if ray_in.direction.dot(rec.normal) > 0. {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * ray_in.direction.dot(rec.normal) / ray_in.direction.norm()
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1. / ref_idx;
                cosine = -ray_in.direction.dot(rec.normal) / ray_in.direction.norm()
            }

            let scattered = match refract(ray_in.direction, outward_normal, ni_over_nt) {
                Some(refracted) => {
                    if random_double(rng) > schlick(cosine, ref_idx) {
                        refracted
                    } else {
                        reflected
                    }
                }
                None => reflected,
            };

            Some((Ray::new(rec.point, scattered, 0.), attenuation))
        }
    }
}
