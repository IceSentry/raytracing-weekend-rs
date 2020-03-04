use crate::{aabb::AABB, material::Material, ray::Ray, vec3::Vec3};

pub mod bvh_node;
pub mod enums;
pub mod hittable_list;
pub mod moving_sphere;
pub mod sphere;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
