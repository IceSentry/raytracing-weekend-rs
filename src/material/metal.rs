use rand::rngs::ThreadRng;

use crate::{
    hittable::HitRecord,
    material::{random_in_unit_sphere, reflect, Material},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
        let fuzz = if self.fuzz < 1. { self.fuzz } else { 1. };

        let reflected = reflect(ray_in.direction.unit(), rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + fuzz * random_in_unit_sphere(rng),
            ray_in.time,
        );

        if scattered.direction.dot(rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
