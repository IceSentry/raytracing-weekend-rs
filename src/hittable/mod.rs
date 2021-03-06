#![allow(clippy::new_ret_no_self)]

use derive_new::*;
use enum_dispatch::enum_dispatch;

use crate::{
    hittable::{
        aabb::AABB, box_rect::BoxRect, bvh_node::BvhNode, constant_medium::ConstantMedium,
        flip_normals::FlipNormals, hittable_list::HittableList, moving_sphere::MovingSphere,
        rect::Rect, rotate::RotateY, sphere::Sphere, translate::Translate,
    },
    material::MaterialType,
    ray::Ray,
    vec3::Vec3,
};

pub mod aabb;
pub mod box_rect;
pub mod bvh_node;
pub mod constant_medium;
pub mod flip_normals;
pub mod hittable_list;
pub mod moving_sphere;
pub mod rect;
pub mod rotate;
pub mod sphere;
pub mod translate;

#[derive(new)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: &'a MaterialType,
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
    Rect,
    FlipNormals,
    BoxRect,
    Translate,
    RotateY,
    ConstantMedium,
}

pub fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::{FRAC_PI_2, PI};

    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + FRAC_PI_2) / PI;

    (u, v)
}
