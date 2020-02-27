use std::fs::File;
use std::io::prelude::*;

mod vec3;
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let mut file = File::create("out.ppm")?;

    let nx = 200;
    let ny = 100;

    write!(file, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3 {
                x: i as f32 / nx as f32,
                y: j as f32 / ny as f32,
                z: 0.2,
            };
            let irgb = 255.99 * col;
            writeln!(
                file,
                "{} {} {}",
                irgb.x as i32, irgb.y as i32, irgb.z as i32
            )?;
        }
    }

    Ok(())
}
