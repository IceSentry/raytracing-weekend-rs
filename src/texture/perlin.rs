use crate::{random::random_double, vec3::Vec3};
use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone)]
pub struct Perlin;

lazy_static! {
    pub static ref VECS: Vec<Vec3> = perlin_generate(&mut thread_rng());
    pub static ref PERM_X: Vec<u8> = perlin_generate_perm(&mut thread_rng());
    pub static ref PERM_Y: Vec<u8> = perlin_generate_perm(&mut thread_rng());
    pub static ref PERM_Z: Vec<u8> = perlin_generate_perm(&mut thread_rng());
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

fn perlin_generate(rng: &mut ThreadRng) -> Vec<Vec3> {
    let mut result = Vec::with_capacity(256);
    for _ in 0..=255 {
        result.push(
            Vec3::new(
                2. * random_double(rng) - 1.,
                2. * random_double(rng) - 1.,
                2. * random_double(rng) - 1.,
            )
            .unit(),
        );
    }
    result
}

fn permute(p: &mut Vec<u8>, rng: &mut ThreadRng) {
    for i in (1..256).rev() {
        let target = rng.gen_range(0, i);
        p.swap(i, target);
    }
}

fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<u8> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i as u8);
    }
    permute(&mut p, rng);
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
