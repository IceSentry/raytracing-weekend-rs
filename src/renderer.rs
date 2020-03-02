use std::f32;

use rand::rngs::ThreadRng;
use rayon::prelude::*;

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::scatter,
    random::random_double,
    ray::Ray,
    vec3::Vec3,
    HEIGHT, WIDTH,
};

const MAX_DEPTH: i32 = 50;

fn _color_iterative(r: &Ray, world: &dyn Hittable, depth: i32, rng: &mut ThreadRng) -> Vec3 {
    let mut local_depth = depth;
    let mut col = {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
    };
    let mut rr = *r;

    while local_depth < 50 {
        match world.hit(&rr, 0.0001, f32::MAX) {
            Some(hit) => {
                if let Some((scattered, attenuation)) = scatter(&rr, &hit, rng) {
                    rr = scattered;
                    col = attenuation * col;
                }
            }
            None => break,
        }
        local_depth += 1;
    }
    col
}

fn color(r: &Ray, world: &dyn Hittable, depth: i32, rng: &mut ThreadRng) -> Vec3 {
    match world.hit(r, 0.0001, f32::MAX) {
        Some(hit) => {
            if depth < MAX_DEPTH {
                if let Some((scattered, attenuation)) = scatter(r, &hit, rng) {
                    return attenuation * color(&scattered, world, depth + 1, rng);
                }
            }
            Vec3::zero()
        }
        None => {
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

pub fn render(cam: Camera, world: HittableList, num_samples: i32) -> Vec<u8> {
    (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, screen_pos| {
            let i = screen_pos % WIDTH;
            let j = HEIGHT - 1 - screen_pos / WIDTH; // reverse the height index

            let mut col = Vec3::zero();
            for _ in 0..num_samples {
                let u = (i as f32 + random_double(rng)) / WIDTH as f32;
                let v = (j as f32 + random_double(rng)) / HEIGHT as f32;
                let ray = cam.get_ray(u, v, rng);
                col += color(&ray, &world, 0, rng);
            }
            col /= num_samples as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let vrgb = 255.99 * col;

            vec![vrgb.x as u8, vrgb.y as u8, vrgb.z as u8, 0xff]
        })
        .flatten()
        .collect()
}
