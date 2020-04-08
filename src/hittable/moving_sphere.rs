use crate::{
    hittable::{
        aabb::{surrounding_box, AABB},
        get_sphere_uv, HitRecord, Hittable,
    },
    material::MaterialType,
    ray::Ray,
    vec3::Vec3,
};
use std::ops::Range;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time: Range<f32>,
    pub radius: f32,
    pub material: MaterialType,
}

impl Hittable for MovingSphere {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let create_rec = |t: f32| -> Option<HitRecord> {
                let point = r.point_at(t);
                let normal = (point - self.center(r.time)) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                Some(HitRecord::new(t, u, v, point, normal, &self.material))
            };

            let mut t = (-b - discriminant.sqrt()) / a;

            if t < t_max && t > t_min {
                return create_rec(t);
            }

            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return create_rec(t);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB {
            min: self.center(t0) - radius,
            max: self.center(t0) + radius,
        };
        let box1 = AABB {
            min: self.center(t1) - radius,
            max: self.center(t1) + radius,
        };
        Some(surrounding_box(box0, box1))
    }
}

impl MovingSphere {
    fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time.start) / (self.time.end - self.time.start))
                * (self.center1 - self.center0)
    }
}
