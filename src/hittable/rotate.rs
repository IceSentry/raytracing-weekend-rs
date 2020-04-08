use super::{aabb::AABB, Hittable, Hittables};
use crate::{
    ray::Ray,
    vec3::{Vec3, Vec3Wrapper},
};

#[derive(Clone)]
pub struct RotateY {
    ptr: Box<Hittables>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(p: Hittables, angle: f32) -> Hittables {
        use std::f32;
        let radians = (f32::consts::PI / 180.) * angle;
        let (sin_theta, cos_theta) = radians.sin_cos();

        let bbox = p.bounding_box(0., 1.);

        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = Vec3::newi(i, j, k);
                    let xyz = ijk * bbox.unwrap().max + ijk.map(|n| 1. - n) * bbox.unwrap().min;

                    let new_x = cos_theta * xyz.x() + sin_theta * xyz.z();
                    let new_z = -sin_theta * xyz.x() + cos_theta * xyz.z();
                    let tester = Vec3::new(new_x, xyz.y(), new_z);

                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }

                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                    }
                }
            }
        }

        let bbox = Some(AABB { min, max });

        Hittables::from(RotateY {
            ptr: Box::new(p),
            sin_theta,
            cos_theta,
            bbox,
        })
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.set_x(self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z());
        origin.set_z(self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z());
        direction.set_x(self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z());
        direction.set_z(self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z());

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.point;
            let mut normal = rec.normal;

            p.set_x(self.cos_theta * rec.point.x() + self.sin_theta * rec.point.z());
            p.set_z(-self.sin_theta * rec.point.x() + self.cos_theta * rec.point.z());
            normal.set_x(self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z());
            normal.set_z(-self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z());

            let mut rec = rec;
            rec.point = p;
            rec.normal = normal;
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        self.bbox
    }
}
