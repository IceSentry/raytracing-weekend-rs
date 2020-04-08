use super::{HitRecord, Hittable, Hittables};
use crate::{
    material::{Isotropic, MaterialType},
    random::random_double,
    ray::Ray,
    texture::TextureType,
    vec3::{Vec3, Vec3Wrapper},
};
use rand::{rngs::SmallRng, SeedableRng};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<Hittables>,
    density: f32,
    phase_function: MaterialType,
}

impl ConstantMedium {
    pub fn new(boundary: Hittables, density: f32, albedo: TextureType) -> Hittables {
        let phase_function = Isotropic::new(albedo);
        Hittables::from(ConstantMedium {
            boundary: Box::new(boundary),
            density,
            phase_function,
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let rng = &mut SmallRng::from_entropy();

        if let Some(rec1) = self.boundary.hit(r, std::f32::MIN, std::f32::MAX) {
            if let Some(rec2) = self.boundary.hit(r, rec1.t + 0.0001, std::f32::MAX) {
                let mut rec1 = rec1;
                let mut rec2 = rec2;

                if rec1.t < t_min {
                    rec1.t = t_min;
                }

                if rec2.t > t_max {
                    rec2.t = t_max;
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                let hit_distance = -(1. / self.density) * random_double(rng).ln();

                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction.length();
                    let point = r.point_at(t);

                    return Some(HitRecord {
                        t,
                        point,
                        normal: Vec3::newi(1, 0, 0),
                        mat: &self.phase_function,
                        u: 0.0,
                        v: 0.0,
                    });
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<super::aabb::AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
