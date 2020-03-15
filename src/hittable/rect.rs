use super::{aabb::AABB, HitRecord, Hittable};
use crate::{material::MaterialType, vec3::Vec3};
use derive_new::*;

#[derive(Clone, new)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: MaterialType,
}

impl Hittable for XYRect {
    fn hit(&self, r: &crate::ray::Ray, t0: f32, t1: f32) -> Option<super::HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (x - self.y0) / (self.y1 - self.y0);
        let point = r.point_at(t);
        let normal = Vec3::newi(0, 0, 1);

        Some(HitRecord::new(t, u, v, point, normal, &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

#[derive(Clone, new)]
pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: MaterialType,
}

impl Hittable for XZRect {
    fn hit(&self, r: &crate::ray::Ray, t0: f32, t1: f32) -> Option<super::HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = r.point_at(t);
        let normal = Vec3::newi(0, 1, 0);

        Some(HitRecord::new(t, u, v, point, normal, &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}

#[derive(Clone, new)]
pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: MaterialType,
}

impl Hittable for YZRect {
    fn hit(&self, r: &crate::ray::Ray, t0: f32, t1: f32) -> Option<super::HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t0 || t > t1 {
            return None;
        }
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = r.point_at(t);
        let normal = Vec3::newi(1, 0, 0);

        Some(HitRecord::new(t, u, v, point, normal, &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
