use eframe::egui_wgpu::wgpu;
use crate::renderer::geometry;


pub struct Mesh<'a> {
    label: Option<&'a str>,
    vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    vertex_buffer_len: u64,
    geometry: Vec<Vec<u8>>,
    geometry_len: usize,
    uploaded_chunks: u32,
}

impl<'a> Mesh<'a> {
    pub fn new(buffer_label: Option<&'a str>, gpu_device: &wgpu::Device, vertex_buffer_len: u64) -> Self {
        let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
            label: buffer_label,
            size: vertex_buffer_len,
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
            label: buffer_label,
            vertex_buffer,
            vertex_buffer_layout,
            vertex_buffer_len,
            geometry: vec![],
            geometry_len: 0,
            uploaded_chunks: 0,
        }
    }

    pub fn extend_buffer(&mut self, gpu_device: wgpu::Device) {
        let mut reallocation_needed = false;
        while self.geometry_len as u64 > self.vertex_buffer_len {
            reallocation_needed = true;
            self.vertex_buffer_len *= 2;
        }
        if reallocation_needed {
            let new_vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
                label: self.label,
                size: self.vertex_buffer_len,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vertex_buffer.destroy();
            self.vertex_buffer = new_vertex_buffer;
        }
    }

    pub fn append_verts(&mut self, verts: &[geometry::Vertex]) -> Result<(), String>{
        let newbytes: Vec<u8> = bytemuck::cast_slice(verts).to_vec();

        self.geometry_len += newbytes.len();
        self.geometry.push(newbytes);

        if self.geometry_len as u64 > self.vertex_buffer_len {
            return Err(String::from("Stored geometry has surpassed the allocated vram. You should call extend_buffer()"));
        } else {
            return Ok(());
        }
    }

    // pub fn append_tris(tris: &[geometry::Triangle]) {

    // }

    // pub fn append_quads(quads: &[geometry::Quad]) {

    // }
}

