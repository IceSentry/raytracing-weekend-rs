use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Material,
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
            let create_rec = |t: f32| -> HitRecord {
                let point = r.point_at(t);
                HitRecord {
                    t,
                    point,
                    normal: (point - self.center(r.time)) / self.radius,
                    mat: self.material,
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
}

impl MovingSphere {
    fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
