use std::mem;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        ffmin(box0.min.x, box1.min.x),
        ffmin(box0.min.y, box1.min.y),
        ffmin(box0.min.z, box1.min.z),
    );

    let big = Vec3::new(
        ffmax(box0.max.x, box1.max.x),
        ffmax(box0.max.y, box1.max.y),
        ffmax(box0.max.z, box1.max.z),
    );

    AABB {
        min: small,
        max: big,
    }
}

impl AABB {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<(f32, f32)> {
        let mut tmin = tmin;
        let mut tmax = tmax;
        for a in 0..3 {
            let inv_d = 1. / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0. {
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
