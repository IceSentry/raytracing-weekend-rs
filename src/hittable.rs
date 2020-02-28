use crate::{ray::Ray, vec3::Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = Default::default();

        for hittable in self.list.iter() {
            match hittable.hit(&r, t_min, closest_so_far) {
                None => (),
                Some(rec) => {
                    hit_anything = true;
                    closest_so_far = rec.t;
                    temp_rec = rec;
                }
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
    }
}
