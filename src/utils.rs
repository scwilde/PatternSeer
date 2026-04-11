use bytemuck;
use glam::{Vec2, Vec3};
use std::cmp::PartialOrd;

mod sealed {
    pub trait Float {  }
}

impl sealed::Float for f32 {  }
impl sealed::Float for f64 {  }
pub fn minf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a < b { a } 
    else { b }
}
pub fn maxf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a > b { a } 
    else { b }
}
