use crate::{
    hittable::{
        aabb::{surrounding_box, AABB},
        HitRecord, Hittable, Hittables,
    },
    ray::Ray,
};

#[derive(Clone)]
pub struct HittableList {
    pub list: Vec<Hittables>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for hittable in self.list.iter() {
            if let Some(rec) = hittable.hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.is_empty() {
            return None;
        }

        let mut bounding_box: AABB;
        match self.list.first().unwrap().bounding_box(t0, t1) {
            Some(temp_box) => bounding_box = temp_box,
            None => return None,
        };

        for i in 1..self.list.len() {
            match self.list[i].bounding_box(t0, t1) {
                Some(temp_box) => bounding_box = surrounding_box(bounding_box, temp_box),
                None => return None,
            };
        }

        Some(bounding_box)
    }
}
