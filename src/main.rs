use std::f32;
use std::time::Instant;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use rand::Rng;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p =
            2.0 * Vec3::new(
                rng.gen_range(0., 1.),
                rng.gen_range(0., 1.),
                rng.gen_range(0., 1.),
            ) - Vec3::new(1., 1., 1.);
        if p.squared_norm() < 1. {
            return p;
        }
    }
}

fn color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    match world.hit(r, 0.0001, f32::MAX) {
        Some(hit) => {
            let target = hit.p + hit.normal + random_in_unit_sphere();
            0.5 * color(&Ray::new(hit.p, target - hit.p), world)
        }
        None => {
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn render_to_frame(frame: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let ns = 100;
    let world = HittableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0., 0., -1.),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0., -100.5, -1.),
                radius: 100.,
            }),
        ],
    };
    let cam = Camera::new();
    let mut pixels = frame.chunks_exact_mut(4);
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0., 1.)) / WIDTH as f32;
                let v = (j as f32 + rng.gen_range(0., 1.)) / HEIGHT as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;

            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let vrgb = 255.99 * col;

            // Draw pixel
            if let Some(pixel) = pixels.next() {
                let rgba = [vrgb.x as u8, vrgb.y as u8, vrgb.z as u8, 0xff];
                pixel.copy_from_slice(&rgba)
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let scale_factor = 3;
    let scaled_width = WIDTH * scale_factor;
    let scaled_height = HEIGHT * scale_factor;

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(scaled_width as f64, scaled_height as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
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

    render_to_frame(pixels.get_frame());

    let mut time = Instant::now();
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

        let now = Instant::now();
        let delta_time = now.duration_since(time);
        time = now;

        // let sync_rate = std::time::Duration::from_millis(15);
        // if delta_time < sync_rate {
        //     std::thread::sleep(sync_rate - delta_time);
        // }

        // let title = format!("Hello Pixels - {:?}", delta_time);
        // window.set_title(&title);
        // window.request_redraw();
    });
}
