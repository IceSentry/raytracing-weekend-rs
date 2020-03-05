use std::io::prelude::*;
use std::{fs::File, time::Instant};

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use structopt::StructOpt;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod renderer;
mod scenes;
mod texture;
mod vec3;

use crate::{
    renderer::render,
    scenes::{random_scene, two_perlin_spheres, two_spheres},
};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

fn init_pixels(window: &Window) -> Pixels {
    let surface = Surface::create(window);
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
    Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Failed to create a new Pixels instance")
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    WindowBuilder::new()
        .with_title("Rendering...")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

fn _render_to_file(pixels: &[u8]) {
    let mut file = File::create("out.ppm").expect("Failed to create file");
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT).expect("Failed to write to file");

    for pixel in pixels.chunks(4) {
        writeln!(file, "{} {} {}", pixel[0], pixel[1], pixel[2]).expect("Failed to write pixel");
    }
}

#[derive(StructOpt, Debug)]
#[structopt(version = "1.0", author = "IceSentry")]
struct Opts {
    /// Number of samples
    #[structopt(short, long, default_value = "8")]
    num_samples: i32,
    #[structopt(short, long, default_value = "16")]
    depth: i32,
    /// Name of the scene to render
    #[structopt(short, long, default_value = "random_scene")]
    scene_name: String,
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::from_args();

    // let mut rng = rand::thread_rng();

    let rng = &mut SmallRng::from_entropy();

    let scene = match opts.scene_name.as_str() {
        "two_spheres" => two_spheres(),
        "two_perlin_spheres" => two_perlin_spheres(),
        "random" => random_scene(rng),
        _ => random_scene(rng),
    };

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);
    let mut pixels = init_pixels(&window);

    let start = Instant::now();

    let rendered_pixels = render(scene.camera, &scene.hittables, opts.num_samples, opts.depth);
    // render_to_file(&rendered_pixels);
    pixels.get_frame().copy_from_slice(&rendered_pixels[..]);

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
