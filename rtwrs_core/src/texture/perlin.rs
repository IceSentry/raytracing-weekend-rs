use crate::{random::random_double, vec3::Vec3};
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, rngs::SmallRng, SeedableRng};

#[derive(Clone)]
pub struct Perlin;

lazy_static! {
    pub static ref VECS: Vec<Vec3> = perlin_generate();
    pub static ref PERM_X: Vec<u8> = perlin_generate_perm();
    pub static ref PERM_Y: Vec<u8> = perlin_generate_perm();
    pub static ref PERM_Z: Vec<u8> = perlin_generate_perm();
}

impl Perlin {
    pub fn noise(&self, p: Vec3) -> f32 {
        let ijk = p.map(f32::floor);
        let uvw = p - ijk;
        let mut corners = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ix = PERM_X[((ijk.x as i32 + di as i32) & 255) as usize];
                    let iy = PERM_Y[((ijk.y as i32 + dj as i32) & 255) as usize];
                    let iz = PERM_Z[((ijk.z as i32 + dk as i32) & 255) as usize];
                    corners[di][dj][dk] = VECS[(ix ^ iy ^ iz) as usize]
                }
            }
        }

        trilinear_interpolate(&corners, uvw)
    }

    pub fn turbulence(&self, p: Vec3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accum.abs()
    }
}

fn perlin_generate() -> Vec<Vec3> {
    let rng = &mut SmallRng::from_entropy();
    let mut result = Vec::with_capacity(256);
    for _ in 0..255 {
        result.push(
            Vec3::new(
                2. * random_double(rng) - 1.0,
                2. * random_double(rng) - 1.0,
                2. * random_double(rng) - 1.0,
            )
            .unit(),
        );
    }
    result
}

fn perlin_generate_perm() -> Vec<u8> {
    let rng = &mut SmallRng::from_entropy();
    let mut p: Vec<u8> = (0..255).collect();
    p.shuffle(rng);
    p
}

fn trilinear_interpolate(corners: &[[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f32 {
    let uvw2 = uvw.map(|x| x * x * (3. - 2. * x));

    let mut accum = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ijk = Vec3::new(i as f32, j as f32, k as f32);
                let weight = uvw - ijk;
                let acc_temp = ijk * uvw2 + (Vec3::from(1.) - ijk) * (Vec3::from(1.) - uvw);
                accum += acc_temp.x * acc_temp.y * acc_temp.z * corners[i][j][k].dot(weight);
            }
        }
    }

    accum
}
