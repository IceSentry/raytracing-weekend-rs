use super::{Texture, TextureType};
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(r: f32, g: f32, b: f32) -> TextureType {
        TextureType::from(ConstantTexture {
            color: Vec3::new(r, g, b),
        })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }
}
