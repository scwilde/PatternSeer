use eframe::{egui_wgpu::{self, wgpu}};
use crate::renderer::mesh::Mesh;
use std::borrow::Cow;

pub mod geometry;
mod mesh;
// mod render_utils;


pub struct RenderContext<'a> {
    render_pipeline: wgpu::RenderPipeline,
    rendered_mesh: Mesh<'a>,
}

pub fn init(wgpu_render_state: &egui_wgpu::RenderState) {
    const MAIN_SHADER: &str = include_str!("shaders/main_shader.wgsl");
    let callback_resources = &mut wgpu_render_state.renderer.write().callback_resources;
    let gpu_device = wgpu_render_state.device.clone();

    let rendered_mesh = Mesh::new(Some("Main Renderer"), &gpu_device, 0);

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
        rendered_mesh
    });
}

// pub fn init(wgpu_render_state: &egui_wgpu::RenderState) {
//     const HELLO_SHADER: &str = include_str!("hello_shader.wgsl");
//     let callback_resources = &mut wgpu_render_state.renderer.write().callback_resources;
//     let gpu_device = wgpu_render_state.device.clone();
//     // Our guess for the initial number of vertices the buffer needs to hold
//     let initial_vertex_count: u64 = 1024;

//     let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
//         label: Some("Rendering Buffer"),
//         size: initial_vertex_count * size_of::<utils::Vertex>() as u64,
//         usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//         mapped_at_creation: false,
//     });
//     let vertex_buffer_layout = wgpu::VertexBufferLayout {
//         array_stride: std::mem::size_of::<utils::Vertex>() as u64,
//         step_mode: wgpu::VertexStepMode::Vertex,
//         attributes: &[
//             // TODO More magic numbers
//             wgpu::VertexAttribute {
//                 offset: 0,
//                 shader_location: 0,
//                 format: wgpu::VertexFormat::Float32x2,
//             },
//             wgpu::VertexAttribute {
//                 offset: std::mem::size_of::<Vec2>() as u64,
//                 shader_location: 1,
//                 format: wgpu::VertexFormat::Float32x3,
//             },
//         ]
//     };

//     let shader_module = gpu_device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("Hello Triangle shader module"),
//         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(HELLO_SHADER)),
//     });

//     let pipeline_layout = gpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: Some("Hello Triangle pipeline layout"),
//         bind_group_layouts: &[],
//         immediate_size: 0,
//     });

//     let render_pipeline = gpu_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: Some("Hello Triangle render pipeline"),
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &shader_module,
//             entry_point: Some("vert_main"),
//             compilation_options: Default::default(),
//             buffers: &[vertex_buffer_layout],
//         },
//         primitive: wgpu::PrimitiveState {
//             topology: wgpu::PrimitiveTopology::TriangleList,
//             ..Default::default()
//         },
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState::default(),
//         fragment: Some(wgpu::FragmentState {
//             module: &shader_module,
//             entry_point: Some("frag_main"),
//             compilation_options: Default::default(),
//             targets: &[Some(wgpu::ColorTargetState {
//                 format: wgpu_render_state.target_format,
//                 blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                 write_mask: wgpu::ColorWrites::ALL,
//             })],
//         }),
//         multiview_mask: None,
//         cache: None,
//     });

//     callback_resources.insert(GridRendererResources {
//         vertex_buffer,
//         vertex_buffer_len: initial_vertex_count,
//         render_pipeline,
//         vertices: Dirty(vec![]),
//     });
// }