use bytemuck;
use glam::{Vec2, vec2};

use crate::utils;

/// An axis of movement.
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}


/// A color containing a red, green, and blue value.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Color {
    /// Red
    r: f32,
    /// Green
    g: f32,
    /// Blue
    b: f32,
}
impl Color {
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };
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
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Triangle { 
    verts: [Vertex; 3]
}
impl Triangle {
    pub fn new(verts: [Vec2; 3], color: Color) -> Self {
        Self { verts: [
            Vertex { position: verts[0], color },
            Vertex { position: verts[1], color },
            Vertex { position: verts[2], color }
        ] }
    }

    pub fn from_verts(verts: [Vertex; 3]) -> Self { Self { verts } }

    pub fn to_verts(&self) -> [Vertex; 3] {
        self.verts
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Quad { 
    tris: [Triangle; 2],
}
impl Quad {
    pub fn from_bb(bb: utils::Bounds2d<f32>, color: Color) -> Self{
        Self {
            tris: [
                Triangle::new([
                    vec2(bb.x.min, bb.y.min),
                    vec2(bb.x.max, bb.y.min),
                    vec2(bb.x.min, bb.y.max),
                    ], color,
                ),
                Triangle::new([
                    vec2(bb.x.max, bb.y.min),
                    vec2(bb.x.max, bb.y.max),
                    vec2(bb.x.min, bb.y.max)
                    ], color,
                )
            ]
        }
    }
    
    pub fn from_verts(verts: [Vertex; 4]) -> Self {
        Self {
            tris: [
                Triangle::from_verts([verts[0], verts[1], verts[3]]),
                Triangle::from_verts([verts[1], verts[2], verts[3]]),
            ]
        }
    }

    pub fn to_verts(&self) -> [Vertex; 6] {
        [
            self.tris[0].to_verts()[0],
            self.tris[0].to_verts()[1],
            self.tris[0].to_verts()[2],
            self.tris[1].to_verts()[0],
            self.tris[1].to_verts()[1],
            self.tris[1].to_verts()[2],
        ]
    }
}