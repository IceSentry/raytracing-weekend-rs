use super::Texture;
use crate::{utils::clamp, vec3::Vec3};

#[derive(Clone)]
pub struct ImageTexture {
    pub data: Vec<u8>,
    pub nx: u32,
    pub ny: u32,
}

impl Texture for ImageTexture {
    #[allow(clippy::many_single_char_names)]
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let nx = self.nx as f32;
        let ny = self.ny as f32;

        let mut i = (u * nx) as usize;
        let mut j = ((1. - v) * ny - 0.001) as usize;

        i = clamp(i, 0, nx as usize - 1);
        j = clamp(j, 0, ny as usize - 1);

        let index = 3 * i + 3 * nx as usize * j;
        let r = self.data[index] as f32 / 255.;
        let g = self.data[index + 1] as f32 / 255.;
        let b = self.data[index + 2] as f32 / 255.;

        Vec3::new(r, g, b)
    }
}
