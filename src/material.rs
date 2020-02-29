use crate::{hittable::HitRecord, random_in_unit_sphere, ray::Ray, reflect, vec3::Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
}

pub fn scatter(r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    match rec.mat {
        Material::Lambertian(albedo) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            Some((Ray::new(rec.p, target - rec.p), albedo))
        }
        Material::Metal(albedo, f) => {
            let fuzz = if f < 1. { f } else { 1. };

            let reflected = reflect(r_in.direction.unit(), rec.normal);
            let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());
            if scattered.direction.dot(rec.normal) > 0. {
                return Some((scattered, albedo));
            }
            None
        }
    }
}
