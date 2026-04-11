use eframe::{egui, egui_wgpu::{self, wgpu}};
use glam::{Vec3, Vec2};
use crate::{
    camera::Camera,
    pattern::Pattern,
    renderer::{
        RenderContext,
        geometry::{
            Vertex,
            Color,
            Axis,
        },
    },
    utils
};
use std::borrow::Cow;
use std::any::TypeId;

struct Gridline {
    position: f32,
    draw_min: f32,
    draw_max: f32,
    weight: f32,
    axis: Axis,
}
impl Gridline {
    fn draw(&self, camera: &Camera) -> [Vertex; 6] {
        match self.axis {
            Axis::X => {
                let x_clip = ((self.position - camera.position.x) * camera.zoom) / (camera.viewport.x / 2.0);
                let draw_min_clip = ((self.draw_min - camera.position.y) * camera.zoom) / (camera.viewport.y / 2.0);
                let draw_max_clip = ((self.draw_max - camera.position.y) * camera.zoom) / (camera.viewport.y / 2.0);
                let weight = self.weight / (camera.viewport.x / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.x / 2.0);
                let draw_color = Color::BLACK;

                [
                    // Top right
                    Vertex {
                        position: Vec2::new(x_clip - weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    Vertex {
                        position: Vec2::new(x_clip + weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    Vertex {
                        position: Vec2::new(x_clip + weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                    //Bottom left
                    Vertex {
                        position: Vec2::new(x_clip - weight / 2.0, draw_max_clip + corner_fix),
                        color: draw_color,
                    },
                    Vertex {
                        position: Vec2::new(x_clip + weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                    Vertex {
                        position: Vec2::new(x_clip - weight / 2.0, draw_min_clip - corner_fix),
                        color: draw_color,
                    },
                ]
            },
            Axis::Y => {
                let y_clip = ((self.position - camera.position[1]) * camera.zoom) / (camera.viewport[1] / 2.0);
                let draw_min_clip = ((self.draw_min - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                let draw_max_clip = ((self.draw_max - camera.position[0]) * camera.zoom) / (camera.viewport[0] / 2.0);
                let weight = self.weight / (camera.viewport[1] / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.y / 2.0);
                let draw_color = Color::BLACK;

                [
                    // Top right
                    Vertex{
                        position: Vec2::new(draw_min_clip - corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip - weight / 2.0),
                        color: draw_color,
                    },
                    //Bottom left
                    Vertex{
                        position: Vec2::new(draw_min_clip - corner_fix, y_clip + weight / 2.0),
                        color: draw_color,
                    },
                    Vertex{
                        position: Vec2::new(draw_max_clip + corner_fix, y_clip - weight / 2.0),
                        color: draw_color,
                    },
                    Vertex{
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
    axis: Axis,
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
        axis: Axis,
    ) -> Self {
        Self {
            working_min: utils::maxf(render_min, grid_min),
            working_max: utils::minf(render_max, grid_max),
            grid_min,
            grid_max,
            draw_min,
            draw_max,
            step,
            curr: utils::maxf(render_min, grid_min),
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


/// Generates the pattern's grid and provides it to `callback_resources`
/// 
/// # Parameters
/// 
/// * `pattern` - The pattern whos dimensions are used to generate the grid.
/// * `camera` - The camera rendering our pattern.
/// * `frame` - Information about the current egui frame. We use it to get access to `callback_resources`.
pub fn render(render_context: &mut RenderContext, camera: &Camera, pattern: &Pattern) -> GridRendererCallback {
    let mut verts = vec![];
    verts.extend(&[
        Vertex { position: Vec2::new(-1.0,  1.0), color: Color::WHITE },
        Vertex { position: Vec2::new( 1.0,  1.0), color: Color::WHITE },
        Vertex { position: Vec2::new( 1.0, -1.0), color: Color::WHITE },
        Vertex { position: Vec2::new(-1.0,  1.0), color: Color::WHITE },
        Vertex { position: Vec2::new( 1.0, -1.0), color: Color::WHITE },
        Vertex { position: Vec2::new(-1.0, -1.0), color: Color::WHITE },
    ]);

    let grid_step = 10.0_f32.powi((((5.0 / camera.zoom).log10() + 1.0).floor() as i32).max(0));

    // // Calculate min and max grid positions
    let x_grids = GridlineIter::new(
        (camera.position[0] - (camera.viewport[0] / (2.0 * camera.zoom))).ceil(),
        (camera.position[0] + (camera.viewport[0] / (2.0 * camera.zoom))).floor(),
        0.0,
        pattern.stitched_dimensions[0] as f32,
        0.0,
        pattern.stitched_dimensions[1] as f32,
        grid_step,
        Axis::X,
    );
    let y_grids = GridlineIter::new(
        (camera.position[1] - (camera.viewport[1] / (2.0 * camera.zoom))).ceil(),
        (camera.position[1] + (camera.viewport[1] / (2.0 * camera.zoom))).floor(),
        0.0,
        pattern.stitched_dimensions[1] as f32,
        0.0,
        pattern.stitched_dimensions[0] as f32,
        grid_step,
        Axis::Y,
    );

    for x_line in x_grids {
        verts.extend(x_line.draw(camera));
    }

    for y_line in y_grids {
        verts.extend(y_line.draw(camera));
    }

    render_context.rendered_mesh.append_verts(TypeId::of::<GridRendererCallback>(), verts.as_slice())
        .expect("Double dipping memory");

    GridRendererCallback {  }
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
        if let Some(render_context) = &mut callback_resources.get_mut::<RenderContext>() {
            let (vertex_buffer, buffer_pos, vert_bytes) = render_context.rendered_mesh
                .get(&TypeId::of::<Self>()).unwrap();

            queue.write_buffer(vertex_buffer, buffer_pos.offset_bytes, vert_bytes);
        }

        Vec::new()
    }

    fn paint(
            &self,
            _info: egui::PaintCallbackInfo,
            render_pass: &mut wgpu::RenderPass<'static>,
            callback_resources: &egui_wgpu::CallbackResources
        ) {
        if let Some(render_context) = callback_resources.get::<RenderContext>() {
            let (vertex_buffer, buffer_pos, vert_bytes) = render_context.rendered_mesh
                .get(&TypeId::of::<Self>()).unwrap();

            render_pass.set_pipeline(&render_context.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(buffer_pos.offset_bytes..buffer_pos.len_bytes));
            render_pass.draw(buffer_pos.offset_verts..buffer_pos.len_verts, 0..1);
        }
    }
}