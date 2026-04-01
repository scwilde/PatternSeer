use eframe::{egui, egui_wgpu::{self, wgpu}};
use crate::utils::{self, Volatile::{self, *}};
use std::borrow::Cow;

struct PatternRendererResources {
    vertex_buffer: wgpu::Buffer,
    vertex_buffer_len: u64,
    vertices: utils::Volatile<Vec<utils::Vertex>>,
    render_pipeline: wgpu::RenderPipeline
}


pub struct PatternRenderer { }

impl PatternRenderer {
    pub fn init(wgpu_render_state: &egui_wgpu::RenderState) {
        const HELLO_SHADER: &str = include_str!("hello_shader.wgsl");
        let callback_resources = &mut wgpu_render_state.renderer.write().callback_resources;
        let gpu_device = wgpu_render_state.device.clone();

        let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: (size_of::<utils::Vertex>() * 3) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<utils::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,                    // ? Whats this for?
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        };

        let shader_module = gpu_device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Hello Triangle shader module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(HELLO_SHADER))
        });

        let pipeline_layout = gpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Hello Triangle pipeline layout"),
            bind_group_layouts: &[],
            immediate_size: 0
        });

        let render_pipeline = gpu_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Hello Triangle render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vert_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_buffer_layout],
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
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            multiview_mask: None,
            cache: None
        });

        callback_resources.insert(PatternRendererResources {
            vertex_buffer,
            vertex_buffer_len: 3,
            vertices: Dirty(vec![]),
            render_pipeline
        });
    }

    pub fn update(vertices: &[utils::Vertex], frame: &mut eframe::Frame) {
        if let Some(resources) = frame.wgpu_render_state().unwrap()
            .renderer.write()
            .callback_resources.get_mut::<PatternRendererResources>() {
                // TODO Dynamically handle increasing the vertex buffer size
                resources.vertices = Dirty(vertices.to_vec());
        } else {
            panic!("PatternRenderer not initialized!");
        }
    }
    pub fn render() -> PatternRendererCallback { PatternRendererCallback {  } }
}

#[derive(Clone, Copy)]
pub struct PatternRendererCallback { }

impl egui_wgpu::CallbackTrait for PatternRendererCallback {
    fn prepare(
            &self,
            _device: &wgpu::Device,
            queue: &wgpu::Queue,
            _screen_descriptor: &egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut wgpu::CommandEncoder,
            callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = &mut callback_resources.get_mut::<PatternRendererResources>() {
            resources.vertices.if_dirty_clean_with(|vertices| {
                println!("Vertices dirty; Uploading to GPU...");
                queue.write_buffer(&resources.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            });
        }

        Vec::new()
    }

    fn paint(
            &self,
            _info: egui::PaintCallbackInfo,
            render_pass: &mut wgpu::RenderPass<'static>,
            callback_resources: &egui_wgpu::CallbackResources
        ) {
        if let Some(resources) = callback_resources.get::<PatternRendererResources>() {
            render_pass.set_pipeline(&resources.render_pipeline);
            render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
            render_pass.draw(0..resources.vertices.inner().len() as u32, 0..1);
        }
    }
}