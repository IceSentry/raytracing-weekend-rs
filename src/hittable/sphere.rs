use crate::aabb::AABB;
use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Material,
}

impl Hittable for Sphere {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let create_rec = |t: f32| -> HitRecord {
                let point = r.point_at(t);
                HitRecord {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    mat: self.mat.clone(),
                }
            };

            let mut t = (-b - discriminant.sqrt()) / a;

            if t < t_max && t > t_min {
                return Some(create_rec(t));
            }

            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return Some(create_rec(t));
            }
        }
        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        let bounding_box = AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(bounding_box)
    }
}
