use crate::{
    hittable::{
        aabb::{surrounding_box, AABB},
        HitRecord, Hittable, Hittables,
    },
    ray::Ray,
};
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Box<Hittables>,
    pub right: Box<Hittables>,
    pub bounding_box: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.bounding_box.hit(ray, t_min, t_max) {
            Some((t_min, t_max)) => {
                let left_rect = self.left.hit(ray, t_min, t_max);
                let right_rec = self.right.hit(ray, t_min, t_max);

                match (left_rect, right_rec) {
                    (Some(left), Some(right)) => {
                        if left.t < right.t {
                            Some(left)
                        } else {
                            Some(right)
                        }
                    }
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn cmp_sort(a: &Hittables, b: &Hittables, index: usize) -> Ordering {
    match (a.bounding_box(0., 0.), b.bounding_box(0., 0.)) {
        (Some(left), Some(right)) => {
            if left.min[index] - right.min[index] < 0. {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        _ => panic!("no bounding box in BvhNode constructor"),
    }
}

impl BvhNode {
    pub fn new(list: Vec<Hittables>, time0: f32, time1: f32, depth: i32) -> Hittables {
        let axis = depth % 3;
        let mut list = list;
        let n = list.len();

        match axis {
            0 => list.sort_by(|a, b| cmp_sort(a, b, 0)), // x
            1 => list.sort_by(|a, b| cmp_sort(a, b, 1)), // y
            _ => list.sort_by(|a, b| cmp_sort(a, b, 2)), // z
        };

        let (left, right) = match n {
            1 => (list[0].clone(), list[0].clone()),
            2 => (list[0].clone(), list[1].clone()),
            _ => {
                let half = n / 2;
                let left_list = list[..(half as usize)].to_vec();
                let right_list = list[(half as usize)..].to_vec();
                (
                    BvhNode::new(left_list, time0, time1, depth + 1),
                    BvhNode::new(right_list, time0, time1, depth + 1),
                )
            }
        };

        match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (Some(box_left), Some(box_right)) => Hittables::from(BvhNode {
                left: Box::new(left),
                right: Box::new(right),
                bounding_box: surrounding_box(box_left, box_right),
            }),
            _ => panic!("no bounding box in BvhNode constructor"),
        }
    }
}
