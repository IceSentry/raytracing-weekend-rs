#[macro_use]
extern crate impl_ops;

use std::f32;
use std::time::Instant;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use rand::Rng;
use rayon::prelude::*;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::{scatter, Material},
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn _color_iterative(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
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
                if let Some((scattered, attenuation)) = scatter(&rr, &hit) {
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

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    match world.hit(r, 0.0001, f32::MAX) {
        Some(hit) => {
            if depth < 50 {
                if let Some((scattered, attenuation)) = scatter(r, &hit) {
                    return attenuation * color(&scattered, world, depth + 1);
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

fn init_world() -> HittableList {
    HittableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0., 0., -1.),
                radius: 0.5,
                mat: Material::Lambertian {
                    albedo: Vec3::new(0.1, 0.2, 0.5),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0., -100.5, -1.),
                radius: 100.,
                mat: Material::Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(1., 0., -1.),
                radius: 0.5,
                mat: Material::Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzziness: 0.,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(-1., 0., -1.),
                radius: 0.5,
                mat: Material::Dielectric { ref_idx: 1.5 },
            }),
            Box::new(Sphere {
                center: Vec3::new(-1., 0., -1.),
                radius: -0.45,
                mat: Material::Dielectric { ref_idx: 1.5 },
            }),
        ],
    }
}

fn render_to_frame(frame: &mut [u8]) {
    let ns = 100;

    let cam = {
        let lookfrom = Vec3::new(3., 3., 2.);
        let lookat = Vec3::new(0., 0., -1.);
        let dist_to_focus = (lookfrom - lookat).norm();
        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            20.,
            WIDTH as f32 / HEIGHT as f32,
            2.0,
            dist_to_focus,
        )
    };

    let world = init_world();

    let pixels: Vec<u8> = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..WIDTH)
                .into_par_iter()
                .map_init(rand::thread_rng, |rng, i| {
                    let mut col = Vec3::zero();
                    for _ in 0..ns {
                        let u = (i as f32 + rng.gen_range(0., 1.)) / WIDTH as f32;
                        let v = (j as f32 + rng.gen_range(0., 1.)) / HEIGHT as f32;
                        let r = cam.get_ray(u, v);
                        col += color(&r, &world, 0);
                    }
                    col /= ns as f32;
                    col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

                    let vrgb = 255.99 * col;

                    vec![vrgb.x as u8, vrgb.y as u8, vrgb.z as u8, 0xff]
                })
                .flatten()
                .collect::<Vec<u8>>()
        })
        .collect();

    frame.copy_from_slice(&pixels[..]);
}

fn main() -> Result<(), Error> {
    let scale_factor = 1;
    let scaled_width = WIDTH * scale_factor;
    let scaled_height = HEIGHT * scale_factor;

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(scaled_width as f64, scaled_height as f64);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(scaled_width, scaled_height, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Failed to create a new Pixels instance")
    };

    let start = Instant::now();
    render_to_frame(pixels.get_frame());
    let end = Instant::now();
    let time_to_render = end.duration_since(start);

    window.set_title(&format!("Hello Pixels - {:?}", time_to_render));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                pixels.render();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => pixels.resize(new_size.width, new_size.height),
            _ => (),
        }
    });
}
