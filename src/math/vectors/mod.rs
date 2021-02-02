// internal imports
pub mod vec2d;
pub mod vec3d;
pub mod vec4d;
pub use super::fct::fast_isqrt;
pub use vec2d::Vec2D;
pub use vec3d::Vec3D;
pub use vec4d::Vec4D;

// pub trait Vector : std::ops::Mul<f32> + Nomalizable {
//     fn null() -> Self;
    
//     fn dot(&self, other : Self) -> f32;

//     fn normalize(&mut self) {
//         self = *self * self.inv_norm();
//     }

//     fn rotate(&mut self, angle : f32, normal : Self);

// }

// pub trait Nomalizable {
//     fn sq_norm(&self) -> f32;

//     fn norm(&self) -> f32 {
//         self.norm().sqrt()
//     }

//     fn inv_norm(&self) -> f32 {
//         fast_isqrt(self.norm())
//     }
// }

// pub trait Conjugate {
//     fn conj(&mut self);
// }

