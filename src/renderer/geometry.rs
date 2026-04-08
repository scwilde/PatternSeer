use bytemuck;
use glam::Vec2;


#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

/// A single point in 2D space containing a position and color.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Vertex {
    /// X and Y position of this vertex in space.
    pub position: Vec2,
    /// red, green, and blue color values of this vertex.
    pub color: Color,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };
}