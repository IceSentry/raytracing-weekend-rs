use std::{ops, ops::{Neg, Index}};

use impl_ops::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::new(0., 0., 0.)
    }

    pub fn one() -> Self {
        Vec3::new(1., 1., 1.)
    }

    pub fn squared_norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// length()
    #[inline]
    pub fn norm(&self) -> f32 {
        self.squared_norm().sqrt()
    }

    pub fn make_unit(&mut self) {
        let k = 1.0 / self.norm();

        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    #[inline]
    pub fn dot(&self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        *self / self.norm()
    }

    /// Applies `f` to each element of the vector in turn, giving a new vector.
    #[inline]
    pub fn map(self, mut f: impl FnMut(f32) -> f32) -> Self {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

/// Broadcasts a single value to all vector lanes.
impl From<f32> for Vec3 {
    #[inline]
    fn from(v: f32) -> Self {
        Vec3::new(v, v, v)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _=> unreachable!(),
        }
    }
}

impl_op!(+ |a: Vec3, b: Vec3| -> Vec3 { 
    Vec3 {
        x: a.x + b.x, 
        y: a.y + b.y, 
        z: a.z + b.z
    } 
});

impl_op!(+ |a: Vec3, b: f32| -> Vec3 { 
    Vec3 {
        x: a.x + b, 
        y: a.y + b, 
        z: a.z + b
    } 
});

impl_op!(+= |a: &mut Vec3, b: Vec3| { 
    *a = *a + b
});

impl_op!(-|a: Vec3, b: Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op!(-|a: Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
    }
});

impl_op!(-= |a: &mut Vec3, b: Vec3| { 
    *a = *a - b
});

impl_op!(*|a: Vec3, b: Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});

impl_op!(*= |a: &mut Vec3, b: Vec3| { 
    *a = *a * b
});

impl_op!(*|a: Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op!(*= |a: &mut Vec3, b: f32| { 
    *a = *a * b
});

impl_op!(*|a: f32, b: Vec3| -> Vec3 { b * a });

impl_op!(/|a: Vec3, b: Vec3| -> Vec3 {
    Vec3 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
    }
});

impl_op!(/|a: f32, b: Vec3| -> Vec3 {
   Vec3 {
       x: a / b.x,
       y: a / b.y,
       z: a / b.z,
   }
});

impl_op!(/|a: Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});

impl_op!(/= |a: &mut Vec3, b: f32| { 
    *a = *a / b
});

impl_op!(/= |a: &mut Vec3, b: Vec3| { 
    *a = *a / b
});

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}
