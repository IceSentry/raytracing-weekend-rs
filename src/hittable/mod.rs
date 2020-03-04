use crate::{
    hittable::{
        aabb::AABB, bvh_node::BvhNode, hittable_list::HittableList, moving_sphere::MovingSphere,
        sphere::Sphere,
    },
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use enum_dispatch::enum_dispatch;

pub mod aabb;
pub mod bvh_node;
pub mod hittable_list;
pub mod moving_sphere;
pub mod sphere;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Material,
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[enum_dispatch(Hittable)]
#[derive(Clone)]
pub enum Hittables {
    Sphere,
    MovingSphere,
    HittableList,
    BvhNode,
}
