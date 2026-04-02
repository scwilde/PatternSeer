use eframe::{egui, egui_wgpu::{self, wgpu}};
use crate::{camera::Camera, utils::{self, Vertex, Volatile::{self, *}}};
use std::borrow::Cow;


/// Used for the storage of resources between render stages.
struct PatternRendererResources {
    /// Reference to the vertex buffer we set up for rendering.
    vertex_buffer: wgpu::Buffer,
    /// Length of the vertex buffer in vertices.
    vertex_buffer_len: u64,
    /// Our configured render pipeline with our shaders and such.
    render_pipeline: wgpu::RenderPipeline,

    /// The list of vertices to be submitted to the buffer before drawing.
    vertices: Vec<utils::Vertex>,
}


/// Handles all of the heavy rendering tasks or delegates them to GPU.
pub struct PatternRenderer { }

impl PatternRenderer {
    /// Initializes the stored `PatternRendererResources` thats accessible to other render stages from `callback_resources`.
    /// 
    /// # Parameters
    /// 
    /// * `wgpu_render_state` - The WGPU context that contains the `callback_resources` we will insert into.
    pub fn init(wgpu_render_state: &egui_wgpu::RenderState) {
        const HELLO_SHADER: &str = include_str!("hello_shader.wgsl");
        let callback_resources = &mut wgpu_render_state.renderer.write().callback_resources;
        let gpu_device = wgpu_render_state.device.clone();

        let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: (size_of::<utils::Vertex>() * 3 * 10) as u64,                 // TODO Magic numbers
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<utils::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // TODO More magic numbers
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
            render_pipeline,
            vertices: vec![],
        });
    }

    /// Renders the given triangle and camera into vertex data to be drawn on screen.
    /// 
    /// # Parameters
    /// 
    /// * `triangle` - The triangle that needs to be rendered.
    /// * `camera` - The camera rendering our scene.
    /// * `frame` - Information about the current egui frame. We use it to get access to `callback_resources`
    /// where we store our rendered geometry
    /// 
    /// # Returns
    /// 
    /// An instance of `PatternRendererCallback` which egui then uses to paint what we rendered
    pub fn render(triangle: &utils::Triangle, camera: &Camera, frame: &mut eframe::Frame) -> PatternRendererCallback {
        if let Some(resources) = frame.wgpu_render_state().unwrap()
            .renderer.write()
            .callback_resources.get_mut::<PatternRendererResources>() {    
                // TODO check that the geometry being rendered is actually changed and only update the vertices if it has been
                // TODO For the grid just check if the camera has changed and if any lines entered or exited

                let mut vertices = vec![];
                for vert in triangle.vertices {
                    let x = ((vert.position[0] - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                    let y = ((vert.position[1] - camera.position[1]) * camera.zoom) / (camera.viewport[1] / 2.0);

                    vertices.push(Vertex {position: [x, y], color: vert.color});
                }
                resources.vertices = vertices;

                PatternRendererCallback {  }
        } else {
            panic!("PatternRenderer not initialized!");
        }
    }
}

/// Callback struct used by egui to draw our rendered scene into a panel.
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
            // TODO expand buffer if nessesary
            // TODO only write vertices if they are changed

            queue.write_buffer(&resources.vertex_buffer, 0, bytemuck::cast_slice(&resources.vertices));
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
            render_pass.draw(0..resources.vertices.len() as u32, 0..1);
        }
    }
}