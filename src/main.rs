use std::fs::File;
use std::io::prelude::*;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod ray;
use ray::Ray;
mod vec3;
use vec3::{Vec3, RGB};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;
const BOX_SIZE: i16 = 64;

/// t^2 * dot(B, B) + 2t * dot(B, A − C) + dot(A − C, A − C) − R^2 = 0
fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    discriminant > 0.
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0., 0., -1.), 0.5, &r) {
        return Vec3::new(1., 0., 0.);
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn render_to_file() {
    let mut file = File::create("out.ppm").expect("Failed to open file");

    let nx = WIDTH;
    let ny = HEIGHT;

    write!(file, "P3\n{} {}\n255\n", nx, ny).expect("Failed to write");

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(r);
            let irgb = RGB::from(255.99 * col);
            writeln!(
                file,
                "{} {} {}",
                irgb.r as i32, irgb.g as i32, irgb.b as i32
            )
            .expect("Failed to write");
        }
    }
}

fn render_to_frame(frame: &mut [u8]) {
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i16;
        let y = (i / WIDTH as usize) as i16;

        let u = x as f32 / WIDTH as f32;
        let v = y as f32 / HEIGHT as f32;
        let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let col = color(r);
        let vrgb = 255.99 * col;

        let rgba = [vrgb.x as u8, vrgb.y as u8, vrgb.z as u8, 0xff];
        pixel.copy_from_slice(&rgba);
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
    });
}
