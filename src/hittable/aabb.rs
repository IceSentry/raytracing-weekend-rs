use std::mem;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = Vec3::min(box0.min, box1.min);
    let max = Vec3::max(box0.max, box1.max);
    AABB { min, max }
}

impl AABB {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<(f32, f32)> {
        let mut tmin = tmin;
        let mut tmax = tmax;
        let inv_d = ray.direction.reciprocal();
        for i in 0..3 {
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d[i];
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d[i];
            if inv_d[i] < 0. {
                mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return None;
            }
        }
        Some((tmin, tmax))
    }
}
