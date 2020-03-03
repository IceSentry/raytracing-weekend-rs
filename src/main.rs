#[macro_use]
extern crate impl_ops;

use std::time::Instant;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use rand::rngs::ThreadRng;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod renderer;
mod vec3;

use crate::{
    camera::Camera,
    hittable::{moving_sphere::MovingSphere, sphere::Sphere, HittableList},
    material::Material,
    random::random_double,
    renderer::render,
    vec3::Vec3,
};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

fn init_world(rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0., -1000., 0.),
                radius: 1000.,
                mat: Material::Lambertian {
                    albedo: Vec3::new(0.5, 0.5, 0.5),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0., 1., 0.),
                radius: 1.,
                mat: Material::Dielectric { ref_idx: 1.5 },
            }),
            Box::new(Sphere {
                center: Vec3::new(-4., 1., 0.),
                radius: 1.,
                mat: Material::Lambertian {
                    albedo: Vec3::new(0.4, 0.2, 0.1),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(4., 1., 0.),
                radius: 1.,
                mat: Material::Metal {
                    albedo: Vec3::new(0.7, 0.6, 0.5),
                    fuzziness: 0.,
                },
            }),
        ],
    };

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let center = Vec3::new(
                a as f32 + 0.9 * random_double(rng),
                0.2,
                b as f32 + 0.9 * random_double(rng),
            );

            if (center - Vec3::new(4., 0.2, 0.)).norm() > 0.9 {
                let material = match random_double(rng) {
                    x if (0.0..0.8).contains(&x) => Material::Lambertian {
                        albedo: Vec3::new(
                            random_double(rng) * random_double(rng),
                            random_double(rng) * random_double(rng),
                            random_double(rng) * random_double(rng),
                        ),
                    },
                    x if (0.8..0.95).contains(&x) => Material::Metal {
                        albedo: Vec3::new(
                            0.5 * (1. + random_double(rng)),
                            0.5 * (1. + random_double(rng)),
                            0.5 * (1. + random_double(rng)),
                        ),
                        fuzziness: 0.5 * random_double(rng),
                    },
                    _ => Material::Dielectric { ref_idx: 1.5 },
                };

                let radius = 0.2;
                world.list.push(match material {
                    Material::Lambertian { .. } => Box::new(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0., 0.5 * random_double(rng), 0.),
                        time0: 0.,
                        time1: 1.0,
                        radius,
                        material,
                    }),
                    _ => Box::new(Sphere {
                        center,
                        radius,
                        mat: material,
                    }),
                });
            }
        });
    });

    world
}

fn init_camera() -> Camera {
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        dist_to_focus,
        0.,
        1.,
    )
}

fn init_pixels(window: &winit::window::Window, scale: u32) -> Pixels {
    let surface = Surface::create(window);
    let surface_texture = SurfaceTexture::new(WIDTH * scale, HEIGHT * scale, surface);
    Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Failed to create a new Pixels instance")
}

fn init_window(event_loop: &EventLoop<()>, scale: u32) -> winit::window::Window {
    let scaled_width = WIDTH * scale;
    let scaled_height = HEIGHT * scale;

    let size = LogicalSize::new(scaled_width as f64, scaled_height as f64);
    WindowBuilder::new()
        .with_title("Rendering...")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

fn render_to_frame(cam: Camera, world: HittableList, ns: i32, frame: &mut [u8]) {
    let pixels = render(cam, world, ns);
    frame.copy_from_slice(&pixels[..]);
}

fn main() -> Result<(), Error> {
    let num_samples = 1;
    let scale = 1;

    let cam = init_camera();
    let world = init_world(&mut rand::thread_rng());

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop, scale);
    let mut pixels = init_pixels(&window, scale);

    let start = Instant::now();
    render_to_frame(cam, world, num_samples, pixels.get_frame());
    let end = Instant::now();
    let time_to_render = end.duration_since(start);

    window.set_title(&format!("Completed in {:?}", time_to_render));

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
