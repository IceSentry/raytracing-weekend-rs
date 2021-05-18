use crate::{
    hittable::{aabb::AABB, get_sphere_uv, HitRecord, Hittable},
    material::MaterialType,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: MaterialType,
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
            let create_hit_rec = |t: f32| -> Option<HitRecord> {
                let point = r.point_at(t);
                let normal = (point - self.center) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                Some(HitRecord::new(t, u, v, point, normal, &self.mat))
            };

            let mut t = (-b - discriminant.sqrt()) / a;

            if t < t_max && t > t_min {
                return create_hit_rec(t);
            }

            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                return create_hit_rec(t);
            }
        }
        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB {
            min: self.center - radius,
            max: self.center + radius,
        })
    }
}
