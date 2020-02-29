use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
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
}
