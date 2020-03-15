use super::{aabb::AABB, HitRecord, Hittable};
use crate::{material::MaterialType, vec3::Vec3};

#[derive(Clone)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: MaterialType,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: MaterialType) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
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
        let v = (x - self.x0) / (self.x1 - self.x0);
        let point = r.point_at(t);
        let normal = Vec3::new(0., 0., 1.);

        Some(HitRecord::new(t, u, v, point, normal, &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
