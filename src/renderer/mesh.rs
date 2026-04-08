use std::{any::TypeId, collections::HashMap};

use eframe::egui_wgpu::{self, wgpu};
use wgpu::Buffer;
use crate::renderer::geometry;

#[derive(Debug)]
pub struct BufferPosition {
    pub offset_bytes: u64,
    pub len_bytes: u64,
    pub offset_verts: u32,
    pub len_verts: u32,
}

pub struct Mesh<'a> {
    gpu_device: wgpu::Device,
    vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    vertex_buffer_size: u64,
    geometry: HashMap<TypeId, (BufferPosition, Vec<u8>)>,
    geometry_len: usize,
    geometry_size: usize,
}

impl<'a> Mesh<'a> {
    pub fn new(gpu_device: &wgpu::Device, initial_size: u64) -> Self {
        let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mesh vertex buffer"),
            size: initial_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<geometry::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Vertex.Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Vertex.Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        Self {
            gpu_device: gpu_device.clone(),
            vertex_buffer,
            vertex_buffer_layout,
            vertex_buffer_size: initial_size,
            geometry: HashMap::new(),
            geometry_size: 0,
            geometry_len: 0,
        }
    }

    pub fn extend_buffer(&mut self) {
        let mut reallocation_needed = false;
        while self.geometry_size as u64 > self.vertex_buffer_size {
            reallocation_needed = true;
            self.vertex_buffer_size *= 2;
            println!("Vertex buffer exceeded, extending buffer to {} bytes", self.vertex_buffer_size);
        }
        if reallocation_needed {
            let new_vertex_buffer = self.gpu_device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Mesh vertex buffer"),
                size: self.vertex_buffer_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vertex_buffer.destroy();
            self.vertex_buffer = new_vertex_buffer;
        }
    }

    pub fn append_verts(
        &mut self,
        bind_callback: TypeId,
        verts: &[geometry::Vertex],
    ) -> Result<(), String> {
        if self.geometry.contains_key(&bind_callback) {
            return Err(format!("The callback type {:?} is already bound to another block of geometry", bind_callback))
        }
        
        let newbytes: Vec<u8> = bytemuck::cast_slice(verts).to_vec();
        let newbytes_len = newbytes.len();
        let offset = self.geometry_size;
        self.geometry.insert(bind_callback, (
            BufferPosition {
                offset_bytes: self.geometry_size as u64,
                len_bytes: newbytes_len as u64,
                offset_verts: self.geometry_len.try_into().expect(""),
                len_verts: verts.len().try_into().expect(""),
            },
            newbytes
        ));
        self.geometry_size += newbytes_len;

        if self.geometry_size as u64 > self.vertex_buffer_size {
            self.extend_buffer();
        }
        Ok(())
    }

    // pub fn append_tris(tris: &[geometry::Triangle]) {

    // }

    // pub fn append_quads(quads: &[geometry::Quad]) {

    // }

    pub fn get(&self, bound_callback: &TypeId) -> Option<(&wgpu::Buffer, &BufferPosition, &[u8])> {
        let (buffer_pos, bytes) = self.geometry.get(bound_callback)?;
        Some((&self.vertex_buffer, buffer_pos, bytes.as_slice()))
    }

    pub fn clear(&mut self) {
        self.geometry.clear();
        self.geometry_size = 0;
    }
}

