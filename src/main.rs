use std::fs::File;
use std::io::prelude::*;

mod ray;
use ray::Ray;
mod vec3;
use vec3::{Vec3, RGB};

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

fn main() -> std::io::Result<()> {
    let mut file = File::create("out.ppm")?;

    let nx = 200;
    let ny = 100;

    write!(file, "P3\n{} {}\n255\n", nx, ny)?;

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
            )?;
        }
    }

    Ok(())
}
