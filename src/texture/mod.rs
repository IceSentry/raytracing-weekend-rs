use crate::{
    texture::{checker_texture::CheckerTexture, constant_texture::ConstantTexture},
    vec3::Vec3,
};
use enum_dispatch::enum_dispatch;

pub mod checker_texture;
pub mod constant_texture;

#[enum_dispatch(Texture)]
#[derive(Clone)]
pub enum TextureType {
    ConstantTexture,
    CheckerTexture,
}

#[enum_dispatch]
pub trait Texture: Clone {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}
