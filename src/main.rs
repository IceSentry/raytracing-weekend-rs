use std::time::Instant;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod aabb;
mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod renderer;
mod scenes;
mod texture;
mod vec3;

use crate::{camera::Camera, hittable::Hittables, renderer::render, scenes::random_scene};
use scenes::two_spheres;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

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

fn render_to_frame(cam: Camera, world: &Hittables, ns: i32, frame: &mut [u8]) {
    let pixels = render(cam, world, ns);
    frame.copy_from_slice(&pixels[..]);
}

fn main() -> Result<(), Error> {
    let num_samples = 10;
    let scale = 1;

    let scene = {
        let mut rng = rand::thread_rng();
        random_scene(&mut rng)
    };

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop, scale);
    let mut pixels = init_pixels(&window, scale);

    let start = Instant::now();
    render_to_frame(
        scene.camera,
        &scene.hittables,
        num_samples,
        pixels.get_frame(),
    );
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
