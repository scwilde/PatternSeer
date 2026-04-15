#![allow(dead_code)]

use std::{any::TypeId, collections::HashMap};
use eframe::egui_wgpu::wgpu;
use crate::renderer::geometry;

/// Contains information about a block of memory in a GPU vertex buffer.
#[derive(Debug)]
pub struct BufferPosition {
    /// Offset of block from the start of the buffer in bytes.
    pub offset_bytes: u64,
    /// Length of the block in bytes.
    pub len_bytes: u64,
    /// Offset position of the block from the start of the buffer in vertices.
    pub offset_verts: u32,
    /// Length of the block in vertices.
    pub len_verts: u32,
}

/// A 2D mesh of rendered geometry.
/// Keeps track of blocks of geometry for ordered rendering.
/// Also manages the attached GPU buffer and ensures it is large enough for all needed geometry.
pub struct Mesh {
    /// GPU device on which the vertex buffer is allocated.
    gpu_device: wgpu::Device,
    /// Allocated vertex buffer.
    vertex_buffer: wgpu::Buffer,
    /// Layout of the vertex buffer's bytes and how they should be enterpreted as vertices.
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'static>,
    /// Number of bytes currently allocated to our vertex buffer.
    vertex_buffer_size: u64,
    /// All the geometry in the mesh. This is a `HashMap` which binds the type ID of a `RendererCallback`
    /// to a `Vec` of bytes as well as the position of that chunk of bytes in the vertex buffer.
    geometry: HashMap<TypeId, (BufferPosition, Vec<u8>)>,
    /// Number of vertices stored in the geometry.
    geometry_len: usize,
    /// Number of total bytes in the geometry.
    geometry_size: usize,
}
impl Mesh {
    /// Creates a new `Mesh` instance.
    ///
    /// # Parameters
    ///
    /// - `gpu_device`: GPU device to allocate a vertex buffer on.
    /// - `initial_size`: Initial size to allocate for the vertex buffer.
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

    /// Repeatedly doubles the allocated size of the vertex buffer until the geometry can fit cleanly inside it.
    pub fn extend_buffer(&mut self) {
        let mut reallocation_needed = false;
        while self.geometry_size as u64 > self.vertex_buffer_size {
            reallocation_needed = true;
            self.vertex_buffer_size *= 2;
        }
        if reallocation_needed {
            println!("Vertex buffer exceeded, extending buffer to {} bytes", self.vertex_buffer_size);
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

    /// Appends some raw bytes to the mesh.
    ///
    /// # Parameters
    ///
    /// - `bind_callback`: `PaintCallback` to bind this block of bytes to.
    /// - `bytes`: Slice of bytes to append to the mesh.
    /// - `num_verts`: The number of vertices that are being appended to the mesh.
    ///
    /// # Returns
    ///
    /// `Result` that can be:
    /// - `Ok`: When the bytes can be appended to the mesh without issue.
    /// - `Err("msg")`: When the `bind_callback` has been previously bound to a block of geometry.
    fn append_bytes(
        &mut self,
        bind_callback: TypeId,
        bytes: &[u8],
        num_verts: usize,
    ) -> Result<(), String> {
        if self.geometry.contains_key(&bind_callback) {
            return Err(format!("The callback type {:?} is already bound to another block of geometry", bind_callback))
        }

        self.geometry.insert(bind_callback, (
            BufferPosition {
                offset_bytes: self.geometry_size as u64,
                len_bytes: bytes.len() as u64,
                offset_verts: self.geometry_len.try_into().expect(""),
                len_verts: num_verts.try_into().expect(""),
            },
            bytes.to_vec(),
        ));
        self.geometry_size += bytes.len();
        self.geometry_len += num_verts;

        if self.geometry_size as u64 > self.vertex_buffer_size {
            self.extend_buffer();
        }

        Ok(())
    }

    /// Appends a slice of `Vertex`s to the mesh.
    ///
    /// # Parameters
    ///
    /// - `bind_callback`: `PaintCallback` to bind this block of geometry to.
    /// - `verts`: Slice of vertices to append to the mesh.
    ///
    /// # Returns
    ///
    /// `Result` that can be:
    /// - `Ok`: When the geometry can be appended to the mesh without issue.
    /// - `Err("msg")`: When the `bind_callback` has been previously bound to a block of geometry.
    pub fn append_verts(&mut self, bind_callback: TypeId, verts: &[geometry::Vertex]) -> Result<(), String> {
        let newbytes: &[u8] = bytemuck::cast_slice(verts);
        self.append_bytes(bind_callback, newbytes, verts.len())?;

        Ok(())
    }

    /// Appends a slice of `Triangle`s to the mesh.
    ///
    /// # Parameters
    ///
    /// - `bind_callback`: `PaintCallback` to bind this block of geometry to.
    /// - `tris`: Slice of triangles to append to the mesh.
    ///
    /// # Returns
    ///
    /// `Result` that can be:
    /// - `Ok`: When the geometry can be appended to the mesh without issue.
    /// - `Err("msg")`: When the `bind_callback` has been previously bound to a block of geometry.
    ///
    /// # Panics
    ///
    /// Always because this method has not been implemented yet.
    pub fn append_tris(&mut self, bind_callback: TypeId, tris: &[geometry::Triangle]) -> Result<(), String> {
        todo!()
    }

    /// Appends a slice of `Quad`s to the mesh.
    ///
    /// # Parameters
    ///
    /// - `bind_callback`: `PaintCallback` to bind this block of geometry to.
    /// - `quads`: Slice of quads to append to the mesh.
    ///
    /// # Returns
    ///
    /// `Result` that can be:
    /// - `Ok`: When the geometry can be appended to the mesh without issue.
    /// - `Err("msg")`: When the `bind_callback` has been previously bound to a block of geometry.
    pub fn append_quads(&mut self, bind_callback: TypeId, quads: &[geometry::Quad]) -> Result<(), String> {
        let newbytes: &[u8] = bytemuck::cast_slice(quads);
        self.append_bytes(bind_callback, newbytes, quads.len() * 6)?;

        Ok(())
    }

    /// Gets a block of geometry and its buffer position from a bound `RendererCallback`'s `TypeID`.
    ///
    /// # Parameters
    ///
    /// - `bound_callback`: `RendererCallback` for which to check for bound geometry.
    ///
    /// # Returns
    ///
    /// `Option` which can be
    /// - `None`: When the specified `RenderCallback` is not bound to any block of geometry.
    /// - `Some`: When the specified `RenderCallback` is bound to geometry, the geometry will be returned in the form:
    ///     (
    ///         `vertex_buffer`: Vertex buffer where the geometry is to be stored.
    ///         `buffer_position`: Offset and length of the geometry in the buffer.
    ///         `vertex_bytes`: Slice containing all the bytes for this block of geometry.
    ///     )
    pub fn get(&self, bound_callback: &TypeId) -> Option<(&wgpu::Buffer, &BufferPosition, &[u8])> {
        let (buffer_pos, bytes) = self.geometry.get(bound_callback)?;
        Some((&self.vertex_buffer, buffer_pos, bytes.as_slice()))
    }

    /// Clears all stored geometry from the mesh. As its meant to be run at the beginning of each frame
    /// it keeps the previously allocated vertex buffer for reuse.
    pub fn clear(&mut self) {
        self.geometry.clear();
        self.geometry_len = 0;
        self.geometry_size = 0;
    }
}
