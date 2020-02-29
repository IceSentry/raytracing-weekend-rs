use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f32 },
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p =
            2.0 * Vec3::new(
                rng.gen_range(0., 1.),
                rng.gen_range(0., 1.),
                rng.gen_range(0., 1.),
            ) - Vec3::new(1., 1., 1.);
        if p.squared_norm() < 1. {
            return p;
        }
    }
}

pub fn scatter(r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    match rec.mat {
        Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            Some((Ray::new(rec.p, target - rec.p), albedo))
        }
        Material::Metal { albedo, fuzziness } => {
            let fuzz = if fuzziness < 1. { fuzziness } else { 1. };

            let reflected = reflect(r_in.direction.unit(), rec.normal);
            let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());
            if scattered.direction.dot(rec.normal) > 0. {
                return Some((scattered, albedo));
            }
            None
        }
    }
}
