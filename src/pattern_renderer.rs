use eframe::{egui, egui_wgpu::{self, wgpu}};
use crate::{camera::Camera, pattern::Pattern, utils::{self, Vertex, Volatile::{self, *}}};
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
    vertices: Volatile<Vec<utils::Vertex>>,
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
        // Our guess for the initial number of vertices the buffer needs to hold
        let initial_vertex_count: u64 = 1024;

        let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rendering Buffer"),
            size: initial_vertex_count * size_of::<utils::Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<utils::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // TODO More magic numbers
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        let shader_module = gpu_device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Hello Triangle shader module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(HELLO_SHADER)),
        });

        let pipeline_layout = gpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Hello Triangle pipeline layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
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
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        callback_resources.insert(PatternRendererResources {
            vertex_buffer,
            vertex_buffer_len: initial_vertex_count,
            render_pipeline,
            vertices: Dirty(vec![]),
        });
    }

    /// Generates the pattern's grid and provides it to `callback_resources`
    /// 
    /// # Parameters
    /// 
    /// * `pattern` - The pattern whos dimensions are used to generate the grid.
    /// * `camera` - The camera rendering our pattern.
    /// * `frame` - Information about the current egui frame. We use it to get access to `callback_resources`.
    pub fn generate_grid(pattern: &Pattern, camera: &Camera, frame: &mut eframe::Frame) {
        if let Some(resources) = frame.wgpu_render_state().unwrap()
            .renderer.write()
            .callback_resources.get_mut::<PatternRendererResources>() {
                // let mut vertices = vec![];
                // for vert in triangle.vertices {
                //     let x = ((vert.position[0] - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                //     let y = ((vert.position[1] - camera.position[1]) * camera.zoom) / (camera.viewport[1] / 2.0);

                //     vertices.push(utils::Vertex {position: [x, y], color: vert.color});
                // }
                // resources.vertices = Dirty(vertices);

                // Calculate min and max grid positions
                let x_min = (camera.position[0] - (camera.viewport[0] / (2.0 * camera.zoom)))
                    .ceil().max(0.0);
                let x_max = (camera.position[0] + (camera.viewport[0] / (2.0 * camera.zoom)))
                    .floor().min(pattern.stitched_dimensions[0] as f32);
                let y_min = (camera.position[1] - (camera.viewport[1] / (2.0 * camera.zoom)))
                    .ceil().max(0.0);
                let y_max = (camera.position[1] + (camera.viewport[1] / (2.0 * camera.zoom)))
                    .floor().min(pattern.stitched_dimensions[1] as f32);
                println!("x: {}, {}; y: {}, {}; total: {}", x_min, x_max, y_min, y_max, (x_max-x_min + y_max-y_min));

                let mut vertices = vec![];
                for x_pos in (x_min as i16)..=(x_max as i16) {
                    let x_pos = x_pos as f32;
                    let x_clip = ((x_pos - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                    let width = 1.0 / (camera.viewport[0] / 2.0);
                    vertices.extend([
                        // Top right
                        utils::Vertex{position: [x_clip - width / 2.0, 1.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [x_clip + width / 2.0, 1.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [x_clip + width / 2.0, -1.0], color: [1.0, 1.0, 1.0]},
                        //Bottom left
                        utils::Vertex{position: [x_clip - width / 2.0, 1.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [x_clip + width / 2.0, -1.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [x_clip - width / 2.0, -1.0], color: [1.0, 1.0, 1.0]},
                    ]);
                }
                for y_pos in (y_min as i16)..=(y_max as i16) {
                    let y_pos = y_pos as f32;
                    let y_clip = ((y_pos - camera.position[1]) * camera.zoom) / (camera.viewport[1] / 2.0);
                    let width = 1.0 / (camera.viewport[1] / 2.0);
                    vertices.extend([
                        // Top right
                        utils::Vertex{position: [-1.0, y_clip + width / 2.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [1.0, y_clip + width / 2.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [1.0, y_clip - width / 2.0], color: [1.0, 1.0, 1.0]},
                        //Bottom left
                        utils::Vertex{position: [-1.0, y_clip + width / 2.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [1.0, y_clip - width / 2.0], color: [1.0, 1.0, 1.0]},
                        utils::Vertex{position: [-1.0, y_clip - width / 2.0], color: [1.0, 1.0, 1.0]},
                    ]);
                }

                resources.vertices = Dirty(vertices);
        } else {
            panic!("PatternRenderer not initialized!");
        }
    }

    /// Provides an instance of `PatternRendererCallback` to give to egui for painting.
    pub fn render() -> PatternRendererCallback{ PatternRendererCallback {  } }
}

/// Callback struct used by egui to draw our rendered scene into a panel.
#[derive(Clone, Copy)]
pub struct PatternRendererCallback { }
impl egui_wgpu::CallbackTrait for PatternRendererCallback {
    fn prepare(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            _screen_descriptor: &egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut wgpu::CommandEncoder,
            callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = &mut callback_resources.get_mut::<PatternRendererResources>() {
            resources.vertices.if_dirty_clean_with(|vertices| {
                println!("CPU vertex buffer dirty, uploading to gpu...");

                // If the vertex buffer is about to overflow, repeatedly double it until large enough
                let mut reallocation_needed = false;
                while vertices.len() as u64 > resources.vertex_buffer_len {
                    reallocation_needed = true;
                    resources.vertex_buffer_len *= 2;
                }
                if reallocation_needed {
                    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Rendering Buffer"),
                        size: resources.vertex_buffer_len * size_of::<utils::Vertex>() as u64,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });
                    resources.vertex_buffer.destroy();
                    resources.vertex_buffer = vertex_buffer;
                }

                // Upload the new vertices to the buffer
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