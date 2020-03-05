use rand::Rng;

pub fn random_double(rng: &mut impl Rng) -> f32 {
    rng.gen_range(0., 1.)
}
