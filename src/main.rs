use std::f32;
use std::time::Instant;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod ray;
use ray::Ray;
mod vec3;
use vec3::Vec3;
mod hittable;
use hittable::{Hittable, HittableList};
mod sphere;
use sphere::Sphere;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    match world.hit(r, 0.0, f32::MAX) {
        Some(hit) => 0.5 * Vec3::new(hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.),
        None => {
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn render_to_frame(frame: &mut [u8]) {
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

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

    let mut pixels = frame.chunks_exact_mut(4);
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = i as f32 / WIDTH as f32;
            let v = j as f32 / HEIGHT as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r, &world);

            let rgb_vector = 255.99 * col;
            let rgba = [
                rgb_vector.x as u8,
                rgb_vector.y as u8,
                rgb_vector.z as u8,
                0xff,
            ];
            match pixels.next() {
                None => return,
                Some(pixel) => pixel.copy_from_slice(&rgba),
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let scale_factor = 2;
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

    let mut time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                render_to_frame(pixels.get_frame());
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

        let title = format!("Hello Pixels - {:?}", delta_time);
        window.set_title(&title);
        window.request_redraw();
    });
}
