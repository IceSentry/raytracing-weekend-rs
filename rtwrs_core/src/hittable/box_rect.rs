use super::{rect::StaticAxis, FlipNormals, Hittable, HittableList, Hittables, Rect, AABB};
use crate::{material::MaterialType, vec3::Vec3};

#[derive(Clone)]
pub struct BoxRect {
    pmin: Vec3,
    pmax: Vec3,
    list_ptr: Box<Hittables>,
}

impl BoxRect {
    pub fn new(p0: Vec3, p1: Vec3, material: MaterialType) -> Hittables {
        let list = HittableList::new(vec![
            Rect::new(
                p0.x()..p1.x(),
                p0.y()..p1.y(),
                p1.z(),
                StaticAxis::Z,
                material.clone(),
            ),
            FlipNormals::new(Rect::new(
                p0.x()..p1.x(),
                p0.y()..p1.y(),
                p0.z(),
                StaticAxis::Z,
                material.clone(),
            )),
            Rect::new(
                p0.x()..p1.x(),
                p0.z()..p1.z(),
                p1.y(),
                StaticAxis::Y,
                material.clone(),
            ),
            FlipNormals::new(Rect::new(
                p0.x()..p1.x(),
                p0.z()..p1.z(),
                p0.y(),
                StaticAxis::Y,
                material.clone(),
            )),
            Rect::new(
                p0.y()..p1.y(),
                p0.z()..p1.z(),
                p1.x(),
                StaticAxis::X,
                material.clone(),
            ),
            FlipNormals::new(Rect::new(
                p0.y()..p1.y(),
                p0.z()..p1.z(),
                p0.x(),
                StaticAxis::X,
                material.clone(),
            )),
        ]);

        Hittables::from(BoxRect {
            pmin: p0,
            pmax: p1,
            list_ptr: Box::new(list),
        })
    }
}

impl Hittable for BoxRect {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        self.list_ptr.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<super::aabb::AABB> {
        Some(AABB {
            min: self.pmin,
            max: self.pmax,
        })
    }
}
