use eframe::{egui, egui_wgpu::{self, wgpu}};
use glam::{Vec3, Vec2};
use crate::{camera::Camera, pattern::Pattern, utils::{self, Vertex, Volatile::{self, *}}};
use std::borrow::Cow;


struct Gridline {
    position: f32,
    draw_min: f32,
    draw_max: f32,
    weight: f32,
    axis: utils::Axis,
}
impl Gridline {
    fn draw(&self, camera: &Camera) -> [utils::Vertex; 6] {
        match self.axis {
            utils::Axis::X => {
                let x_clip = ((self.position - camera.position.x) * camera.zoom) / (camera.viewport.x / 2.0);
                let draw_min_clip = ((self.draw_min - camera.position.y) * camera.zoom) / (camera.viewport.y / 2.0);
                let draw_max_clip = ((self.draw_max - camera.position.y) * camera.zoom) / (camera.viewport.y / 2.0);
                let weight = self.weight / (camera.viewport.x / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.x / 2.0);
                let draw_color = Vec3::new(0.0, 0.0, 0.0);

                [
                    // Top right
                    utils::Vertex{
                        position: Vec2::new(x_clip - weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(x_clip + weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(x_clip + weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                    //Bottom left
                    utils::Vertex{
                        position: Vec2::new(x_clip - weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(x_clip + weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(x_clip - weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                ]
            },
            utils::Axis::Y => {
                let y_clip = ((self.position - camera.position[1]) * camera.zoom) / (camera.viewport[1] / 2.0);
                let draw_min_clip = ((self.draw_min - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                let draw_max_clip = ((self.draw_max - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                let weight = self.weight / (camera.viewport[1] / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.y / 2.0);
                let draw_color = Vec3::new(0.0, 0.0, 0.0);

                [
                    // Top right
                    utils::Vertex{
                        position: Vec2::new(draw_min_clip - corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip - weight / 2.0),
                        color: draw_color,
                    },
                    //Bottom left
                    utils::Vertex{
                        position: Vec2::new(draw_min_clip - corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip - weight / 2.0),
                        color: draw_color,
                    },
                    utils::Vertex{
                        position: Vec2::new(draw_min_clip - corner_fix, y_clip - weight / 2.0),
                        color: draw_color,
                    },
                ]
            }
        }
    }
}

struct GridlineIter {
    working_min: f32,
    working_max: f32,
    grid_min: f32,
    grid_max: f32,
    draw_min: f32,
    draw_max: f32,
    step: f32,
    curr: f32,
    axis: utils::Axis,
    done: bool,
}
impl GridlineIter {
    fn new(
            render_min: f32,
            render_max: f32,
            grid_min: f32,
            grid_max: f32,
            draw_min: f32,
            draw_max: f32,
            step: f32,
            axis: utils::Axis
        ) -> Self {
        Self {
            working_min: utils::maxf32(render_min, grid_min),
            working_max: utils::minf32(render_max, grid_max),
            grid_min,
            grid_max,
            draw_min,
            draw_max,
            step,
            curr: utils::maxf32(render_min, grid_min),
            axis,
            done: false,
        }
    }
}
impl Iterator for GridlineIter {
    type Item = Gridline;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.curr == self.working_min {
            self.curr -= self.curr % self.step;
        }
        let position = self.curr;
        self.curr += self.step;
        self.curr = self.curr.min(self.working_max);

        let weight = match position {
            p if p == self.grid_min || p == self.grid_max => 5.0,
            p if p % (100.0 * self.step) == 0.0 => 5.0,
            p if p % (10.0 * self.step) == 0.0 => 3.0,
            _ => 1.0,
        };

        if position == self.working_max {
            self.done = true;
        }
        Some(Gridline {
            position,
            draw_min: self.draw_min,
            draw_max: self.draw_max,
            weight,
            axis: self.axis,
        })
    }
}


/// Used for the storage of resources between render stages.
struct GridRendererResources {
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
pub struct GridRenderer { }

impl GridRenderer {
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
                    offset: std::mem::size_of::<Vec2>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
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

        callback_resources.insert(GridRendererResources {
            vertex_buffer,
            vertex_buffer_len: initial_vertex_count,
            render_pipeline,
            vertices: Dirty(vec![]),
        });
    }

    pub fn clear_with_color(color: Vec3, frame: &mut eframe::Frame) {
        if let Some(resources) = frame.wgpu_render_state().unwrap()
            .renderer.write()
            .callback_resources.get_mut::<GridRendererResources>() {
                resources.vertices = Dirty(vec![
                    Vertex { position: Vec2::new(-1.0,  1.0), color },
                    Vertex { position: Vec2::new( 1.0,  1.0), color },
                    Vertex { position: Vec2::new( 1.0, -1.0), color },
                    Vertex { position: Vec2::new(-1.0,  1.0), color },
                    Vertex { position: Vec2::new( 1.0, -1.0), color },
                    Vertex { position: Vec2::new(-1.0, -1.0), color },
                ]);
        } else {
            panic!("PatternRenderer not initialized!");
        }
    }

    /// Generates the pattern's grid and provides it to `callback_resources`
    /// 
    /// # Parameters
    /// 
    /// * `pattern` - The pattern whos dimensions are used to generate the grid.
    /// * `camera` - The camera rendering our pattern.
    /// * `frame` - Information about the current egui frame. We use it to get access to `callback_resources`.
    pub fn render_grid(pattern: &Pattern, camera: &Camera, frame: &mut eframe::Frame) {
        if let Some(resources) = frame.wgpu_render_state().unwrap()
            .renderer.write()
            .callback_resources.get_mut::<GridRendererResources>() {
                // TODO Optimization: calculate this only on zoom events and store inside camera
                let grid_step = 10.0_f32.powi((((5.0 / camera.zoom).log10() + 1.0).floor() as i32).max(0));

                // Calculate min and max grid positions
                let x_grids = GridlineIter::new(
                    (camera.position[0] - (camera.viewport[0] / (2.0 * camera.zoom))).ceil(),
                    (camera.position[0] + (camera.viewport[0] / (2.0 * camera.zoom))).floor(),
                    0.0,
                    pattern.stitched_dimensions[0] as f32,
                    0.0,
                    pattern.stitched_dimensions[1] as f32,
                    grid_step,
                    utils::Axis::X,
                );
                let y_grids = GridlineIter::new(
                    (camera.position[1] - (camera.viewport[1] / (2.0 * camera.zoom))).ceil(),
                    (camera.position[1] + (camera.viewport[1] / (2.0 * camera.zoom))).floor(),
                    0.0,
                    pattern.stitched_dimensions[1] as f32,
                    0.0,
                    pattern.stitched_dimensions[0] as f32,
                    grid_step,
                    utils::Axis::Y,
                );

                resources.vertices.dirty_with(|vertices| {
                    for x_line in x_grids {
                        vertices.extend(x_line.draw(camera));
                    }
                    for y_line in y_grids {
                        vertices.extend(y_line.draw(camera));
                    }
                });
        } else {
            panic!("PatternRenderer not initialized!");
        }
    }

    /// Provides an instance of `PatternRendererCallback` to give to egui for painting.
    pub fn get_render() -> GridRendererCallback{ GridRendererCallback {  } }
}

/// Callback struct used by egui to draw our rendered scene into a panel.
#[derive(Clone, Copy)]
pub struct GridRendererCallback { }
impl egui_wgpu::CallbackTrait for GridRendererCallback {
    fn prepare(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            _screen_descriptor: &egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut wgpu::CommandEncoder,
            callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = &mut callback_resources.get_mut::<GridRendererResources>() {
            resources.vertices.if_dirty_clean_with(|vertices| {
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
        if let Some(resources) = callback_resources.get::<GridRendererResources>() {
            render_pass.set_pipeline(&resources.render_pipeline);
            render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
            render_pass.draw(0..resources.vertices.inner().len() as u32, 0..1);
        }
    }
}