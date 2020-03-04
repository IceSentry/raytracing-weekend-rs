use rand::rngs::ThreadRng;

use crate::{
    hittable::HitRecord,
    material::{reflect, refract, schlick, Material},
    random::random_double,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction, rec.normal);
        let ni_over_nt: f32;
        let attenuation = Vec3::new(1., 1., 1.);
        let cosine: f32;

        if ray_in.direction.dot(rec.normal) > 0. {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction.dot(rec.normal) / ray_in.direction.norm()
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1. / self.ref_idx;
            cosine = -ray_in.direction.dot(rec.normal) / ray_in.direction.norm()
        }

        let scattered = match refract(ray_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if random_double(rng) > schlick(cosine, self.ref_idx) {
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
