use super::{aabb::AABB, Hittable, Hittables};
use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct Translate {
    offset: Vec3,
    ptr: Box<Hittables>,
}

impl Translate {
    pub fn new(ptr: Hittables, offset: Vec3) -> Hittables {
        Hittables::from(Translate {
            ptr: Box::new(ptr),
            offset,
        })
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let moved_ray = Ray::new(r.origin - self.offset, r.direction, r.time);
        match self.ptr.hit(&moved_ray, t_min, t_max) {
            Some(rec) => {
                let mut rec = rec;
                rec.point += self.offset;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<super::aabb::AABB> {
        match self.ptr.bounding_box(t0, t1) {
            Some(bbox) => Some(AABB {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            }),
            None => None,
        }
    }
}
