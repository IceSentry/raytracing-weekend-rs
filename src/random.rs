use rand::{rngs::ThreadRng, Rng};

pub fn random_double(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(0., 1.)
}
