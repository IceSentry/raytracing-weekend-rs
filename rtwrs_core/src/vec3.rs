use glam;

pub type Vec3 = glam::Vec3;

pub trait Vec3Wrapper {
    fn newi(x: i32, y: i32, z: i32) -> Self;
    /// Applies `f` to each element of the vector in turn, giving a new vector.
    fn map(&self, f: impl FnMut(f32) -> f32) -> Self;
    /// Applies `f` to each element of the vector in turn, giving a new vector.
    fn map_mut(&mut self, f: impl FnMut(f32) -> f32);
}

impl Vec3Wrapper for Vec3 {
    #[inline]
    fn newi(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3::new(x as f32, y as f32, z as f32)
    }

    /// Applies `f` to each element of the vector in turn, giving a new vector.
    #[inline]
    fn map(&self, mut f: impl FnMut(f32) -> f32) -> Self {
        Vec3::new(f(self.x()), f(self.y()), f(self.z()))
    }

    /// Applies `f` to each element of the vector in turn, giving a new vector.
    #[inline]
    fn map_mut(&mut self, mut f: impl FnMut(f32) -> f32) {
        self.set_x(f(self.x()));
        self.set_y(f(self.y()));
        self.set_z(f(self.z()));
    }
}
