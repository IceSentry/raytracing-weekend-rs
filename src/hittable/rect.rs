use super::{aabb::AABB, HitRecord, Hittable, Hittables};
use crate::{material::MaterialType, vec3::Vec3};
use std::ops::Range;

#[derive(Clone, Copy)]
pub enum StaticAxis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub struct Rect {
    range1: Range<f32>,
    range2: Range<f32>,
    k: f32,
    static_axis: StaticAxis,
    material: MaterialType,
}

impl Rect {
    pub fn new(
        range1: Range<f32>,
        range2: Range<f32>,
        k: f32,
        static_axis: StaticAxis,
        material: MaterialType,
    ) -> Hittables {
        Hittables::from(Rect {
            range1,
            range2,
            k,
            static_axis,
            material,
        })
    }
}

impl Hittable for Rect {
    fn hit(&self, r: &crate::ray::Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let axis_index = match self.static_axis {
            StaticAxis::X => (0, 1, 2),
            StaticAxis::Y => (1, 0, 2),
            StaticAxis::Z => (2, 0, 1),
        };
        let t = (self.k - r.origin[axis_index.0]) / r.direction[axis_index.0];

        if t < t0 || t > t1 {
            return None;
        }

        let axis1 = r.origin[axis_index.1] + t * r.direction[axis_index.1];
        let axis2 = r.origin[axis_index.2] + t * r.direction[axis_index.2];

        if !self.range1.contains(&axis1) || !self.range2.contains(&axis2) {
            return None;
        }

        let u = (axis1 - self.range1.start) / (self.range1.end - self.range1.start);
        let v = (axis2 - self.range2.start) / (self.range2.end - self.range2.start);
        let point = r.point_at(t);
        let mut normal = Vec3::ZERO;
        normal[axis_index.0] = 1.0;

        Some(HitRecord::new(t, u, v, point, normal, &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let aabb = match self.static_axis {
            StaticAxis::X => AABB {
                min: Vec3::new(self.k - 0.0001, self.range1.start, self.range2.start),
                max: Vec3::new(self.k + 0.0001, self.range1.end, self.range2.end),
            },
            StaticAxis::Y => AABB {
                min: Vec3::new(self.range1.start, self.k - 0.0001, self.range2.start),
                max: Vec3::new(self.range1.end, self.k + 0.0001, self.range2.end),
            },
            StaticAxis::Z => AABB {
                min: Vec3::new(self.range1.start, self.range2.start, self.k - 0.0001),
                max: Vec3::new(self.range1.end, self.range2.end, self.k + 0.0001),
            },
        };

        Some(aabb)
    }
}
