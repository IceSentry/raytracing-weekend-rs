use crate::{
    texture::{perlin::Perlin, Texture},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::one() * self.noise.noise(self.scale * p)
    }
}
