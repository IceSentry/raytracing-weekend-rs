use crate::{random::random_double, vec3::Vec3};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Perlin {
            ranvec: perlin_generate(rng),
            perm_x: perlin_generate_perm(rng),
            perm_y: perlin_generate_perm(rng),
            perm_z: perlin_generate_perm(rng),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.z.floor();
        let mut w = p.z - p.z.floor();

        u = u * u * (3. - 2. * u);
        v = v * v * (3. - 2. * v);
        w = w * w * (3. - 2. * w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]) as usize;

                    c[di][dj][dk] = self.ranvec[index];
                }
            }
        }

        trilinear_interpolate(&c, u, v, w)
    }
}

fn perlin_generate(rng: &mut ThreadRng) -> Vec<Vec3> {
    let mut result = Vec::with_capacity(256);
    for _ in 0..256 {
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

fn permute(p: &mut Vec<usize>, rng: &mut ThreadRng) {
    for i in (0..p.len()).rev() {
        let target = rng.gen_range(0, i + 1);
        p.swap(i, target);
    }
}

fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i);
    }
    permute(&mut p, rng);
    p
}

fn trilinear_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    let mut accum = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1 - i) as f32 * (1. - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1. - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1. - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }

    accum
}
