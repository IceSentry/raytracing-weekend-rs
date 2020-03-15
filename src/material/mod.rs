use enum_dispatch::enum_dispatch;
use rand::Rng;

use crate::{
    hittable::HitRecord,
    random::random_double,
    ray::Ray,
    texture::{Texture, TextureType},
    vec3::Vec3,
};
use utils::{random_in_unit_sphere, reflect, refract, schlick};

mod utils;

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)>;

    fn emitted(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[enum_dispatch(Material)]
#[derive(Clone)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}

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

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
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

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
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

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: TextureType,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord, _rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
