use eframe::egui_wgpu::{self, wgpu};
use crate::renderer::mesh::Mesh;
use std::borrow::Cow;

mod geometry;
mod mesh;
pub mod grid_renderer;
pub mod frame_timer;


/// An object containing the render pipeline and mesh for drawing to the screen.
pub struct RenderContext {
    pub render_pipeline: wgpu::RenderPipeline,
    pub rendered_mesh: Mesh,
}

/// Initializes the `RenderContext` object inside of `callback_resources`.
///
/// # Parameters
///
/// - `wgpu_render_state`: Render state of wgpu.
/// Provides the GPU device for the mesh and the reference to `callback_resources` to bind `RenderContext`.
pub fn init(wgpu_render_state: &egui_wgpu::RenderState) {
    const MAIN_SHADER: &str = include_str!("shaders/main_shader.wgsl");
    let callback_resources = &mut wgpu_render_state.renderer.write().callback_resources;
    let gpu_device = wgpu_render_state.device.clone();

    let rendered_mesh = Mesh::new(&gpu_device, 1024);

    let shader_module = gpu_device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Main shader module"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(MAIN_SHADER)),
    });

    let pipeline_layout = gpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Main pipeline layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    let render_pipeline = gpu_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main render pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vert_main"),
            compilation_options: Default::default(),
            buffers: &[rendered_mesh.vertex_buffer_layout.clone()],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("frag_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu_render_state.target_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    });

    callback_resources.insert(RenderContext {
        render_pipeline,
        rendered_mesh,
    });
}
