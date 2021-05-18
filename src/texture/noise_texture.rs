use crate::{
    texture::{perlin::Perlin, Texture},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct NoiseTexture {
    pub perlin: Perlin,
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::ONE * 0.5 * (1. + (self.scale * p.z + 10. * self.perlin.turbulence(p, 7)).sin())
        // Vec3::one() * self.perlin.noise(self.scale * p)
        // Vec3::one() * self.perlin.turbulence(self.scale * p, 7)
        // Vec3::one() * self.perlin.noise(p)
    }
}
