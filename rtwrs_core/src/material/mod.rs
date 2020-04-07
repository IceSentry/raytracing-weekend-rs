#![allow(clippy::new_ret_no_self)]

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
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)>;

    #[allow(unused)]
    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
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
    Isotropic,
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: TextureType,
}

impl Lambertian {
    pub fn new(albedo: TextureType) -> MaterialType {
        MaterialType::from(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rng);
        Some((
            Ray::new(hit.point, target - hit.point, ray.time),
            self.albedo.value(hit.u, hit.v, hit.point),
        ))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        let fuzz = if self.fuzz < 1. { self.fuzz } else { 1. };

        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + fuzz * random_in_unit_sphere(rng),
            ray.time,
        );

        if scattered.direction.dot(hit.normal) > 0. {
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray.direction, hit.normal);
        let attenuation = Vec3::new(1., 1., 1.);
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if ray.direction.dot(hit.normal) > 0. {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.direction.dot(hit.normal) / ray.direction.length()
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1. / self.ref_idx;
            cosine = -ray.direction.dot(hit.normal) / ray.direction.length()
        }

        let scattered = match refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if random_double(rng) > schlick(cosine, self.ref_idx) {
                    refracted
                } else {
                    reflected
                }
            }
            None => reflected,
        };

        Some((Ray::new(hit.point, scattered, 0.), attenuation))
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: TextureType,
}

impl DiffuseLight {
    pub fn new(emit: TextureType) -> MaterialType {
        MaterialType::from(DiffuseLight { emit })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord, _rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic {
    pub albedo: TextureType,
}

impl Isotropic {
    pub fn new(albedo: TextureType) -> MaterialType {
        MaterialType::from(Isotropic { albedo })
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit.point, random_in_unit_sphere(rng), 0.0);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some((scattered, attenuation))
    }
}
