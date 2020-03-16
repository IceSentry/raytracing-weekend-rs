use super::{Hittable, Hittables};

#[derive(Clone)]
pub struct FlipNormals {
    pub ptr: Box<Hittables>,
}

impl FlipNormals {
    pub fn new(ptr: Hittables) -> Self {
        FlipNormals { ptr: Box::new(ptr) }
    }
}

impl Hittable for FlipNormals {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        if let Some(rec) = self.ptr.hit(r, t_min, t_max) {
            let mut rec = rec;
            rec.normal = -rec.normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<super::aabb::AABB> {
        self.ptr.bounding_box(t0, t1)
    }
}
