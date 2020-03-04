use crate::{
    aabb::AABB,
    hittable::{bvh_node, hittable_list::HittableList, moving_sphere, sphere, HitRecord, Hittable},
    ray::Ray,
};

#[derive(Clone)]
pub enum Hittables {
    Sphere(sphere::Sphere),
    MovingSphere(moving_sphere::MovingSphere),
    List(HittableList),
    BvhNode(bvh_node::BvhNode),
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Hittables::MovingSphere(moving_sphere) => moving_sphere.hit(ray, t_min, t_max),
            Hittables::List(list) => list.hit(ray, t_min, t_max),
            Hittables::BvhNode(node) => node.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self {
            Hittables::Sphere(sphere) => sphere.bounding_box(t0, t1),
            Hittables::MovingSphere(moving_sphere) => moving_sphere.bounding_box(t0, t1),
            Hittables::List(list) => list.bounding_box(t0, t1),
            Hittables::BvhNode(node) => node.bounding_box(t0, t1),
        }
    }
}
