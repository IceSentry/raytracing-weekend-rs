use rand::Rng;

use crate::{
    hittable::HitRecord,
    material::{random_in_unit_sphere, Material},
    ray::Ray,
    texture::{Texture, TextureType},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: TextureType,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        let target = rec.point + rec.normal + random_in_unit_sphere(rng);
        Some((
            Ray::new(rec.point, target - rec.point, ray_in.time),
            self.albedo.value(rec.u, rec.v, rec.point),
        ))
    }
}
