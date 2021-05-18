use crate::{
    texture::{Texture, TextureType},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct CheckerTexture {
    pub odd: Box<TextureType>,
    pub even: Box<TextureType>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        match sines {
            s if s < 0. => self.odd.value(u, v, p),
            _ => self.even.value(u, v, p),
        }
    }
}
