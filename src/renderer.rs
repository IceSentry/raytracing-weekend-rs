use std::f32;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::{
    camera::Camera,
    hittable::{Hittable, Hittables},
    material::Material,
    random::random_double,
    ray::Ray,
    vec3::Vec3,
    HEIGHT, WIDTH,
};

fn color(mut ray: Ray, world: &Hittables, max_depth: i32, rng: &mut impl Rng) -> Vec3 {
    let mut color_accumulator = Vec3::zero();
    // {
    //     let t = 0.5 * (ray.direction.unit().y + 1.0);
    //     (1. - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.)
    // }
    let mut bounces = 0;
    let mut strength = Vec3::one();

    while let Some(hit) = world.hit(&ray, 0.001, f32::MAX) {
        let emitted = hit.mat.emitted(hit.u, hit.v, hit.point);
        color_accumulator += strength * emitted;

        if let Some((scattered, attenuation)) = hit.mat.scatter(&ray, &hit, rng) {
            ray = scattered;
            strength *= attenuation;
        }

        bounces += 1;
        if bounces >= max_depth {
            break;
        }
    }
    color_accumulator
}

fn _color(r: &Ray, world: &Hittables, depth: i32, max_depth: i32, rng: &mut impl Rng) -> Vec3 {
    match world.hit(r, 0.001, f32::MAX) {
        Some(hit) => {
            let emitted = hit.mat.emitted(hit.u, hit.v, hit.point);
            if depth < max_depth {
                if let Some((scattered, attenuation)) = hit.mat.scatter(r, &hit, rng) {
                    return emitted
                        + attenuation * _color(&scattered, world, depth + 1, max_depth, rng);
                }
            }
            emitted
        }
        None => Vec3::zero(),
    }
}

pub fn render(cam: Camera, world: &Hittables, num_samples: i32, max_depth: i32) -> Vec<u8> {
    (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map_init(SmallRng::from_entropy, |rng, screen_pos| {
            let i = screen_pos % WIDTH;
            let j = HEIGHT - 1 - screen_pos / WIDTH; // reverse the height index

            let mut col = Vec3::zero();
            for _ in 0..num_samples {
                let u = (i as f32 + random_double(rng)) / WIDTH as f32;
                let v = (j as f32 + random_double(rng)) / HEIGHT as f32;
                let ray = cam.get_ray(u, v, rng);
                col += color(ray, world, max_depth, rng);
            }
            col /= num_samples as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let vrgb = 255.99 * col;

            vec![vrgb.x as u8, vrgb.y as u8, vrgb.z as u8, 0xff]
        })
        .flatten()
        .collect()
}
