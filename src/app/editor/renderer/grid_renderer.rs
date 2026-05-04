use crate::{
    app::{
        editor::{
            camera::Camera,
            renderer::{
                geometry::{
                    Axis,
                    Quad,
                },
                EditorRenderContext,
            },
        },
    },
    pattern::Pattern,
    utils,
};
use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
};
use std::any::TypeId;

/// A quad representing a line in a grid.
struct Gridline {
    /// This gridline's world space position along its axis.
    position: f32,
    /// Minimum and maximum world space positions of the gridline's endpoints to prevent overrunning the pattern.
    endpoints: utils::Bounds<f32>,
    /// Thickness of this gridline.
    weight: f32,
    /// Axis long which this gridline is placed, perpendicular to that which it points.
    axis: Axis,
}
impl Gridline {
    /// Generates the clip space vertices of this gridline.
    ///
    /// # Parameters
    ///
    /// - `camera`: The camera whos positions and viewport are used to calculate clip space.
    ///
    /// # Returns
    ///
    /// A black `Quad` that makes up this gridline.
    fn draw(&self, camera: &Camera) -> Quad {
        match self.axis {
            Axis::X => {
                let x_clip =
                    ((self.position - camera.position.x) * camera.zoom) / (camera.viewport.x / 2.0);
                let endpoints_clip = utils::Bounds {
                    min: ((self.endpoints.min - camera.position.y) * camera.zoom)
                        / (camera.viewport.y / 2.0),
                    max: ((self.endpoints.max - camera.position.y) * camera.zoom)
                        / (camera.viewport.y / 2.0),
                };
                let weight = self.weight / (camera.viewport.x / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.x / 2.0);

                Quad::from_bb(
                    utils::bounds2d([
                        [x_clip - weight / 2.0, x_clip + weight / 2.0],
                        [
                            endpoints_clip.min - corner_fix,
                            endpoints_clip.max + corner_fix,
                        ],
                    ]),
                    utils::Color::BLACK,
                )
            }
            Axis::Y => {
                let y_clip =
                    ((self.position - camera.position.y) * camera.zoom) / (camera.viewport.y / 2.0);
                let endpoints_clip = utils::Bounds {
                    min: ((self.endpoints.min - camera.position.x) * camera.zoom)
                        / (camera.viewport.x / 2.0),
                    max: ((self.endpoints.max - camera.position.x) * camera.zoom)
                        / (camera.viewport.x / 2.0),
                };
                let weight = self.weight / (camera.viewport.y / 2.0);
                let corner_fix = (self.weight / 4.0) / (camera.viewport.y / 2.0);

                Quad::from_bb(
                    utils::bounds2d([
                        [
                            endpoints_clip.min - corner_fix,
                            endpoints_clip.max + corner_fix,
                        ],
                        [y_clip - weight / 2.0, y_clip + weight / 2.0],
                    ]),
                    utils::Color::BLACK,
                )
            }
        }
    }
}

/// An iterator which generates all `Gridline`s along an axis between two world space positions.
struct GridlineIter {
    /// Minimum gridline in world space to draw along axis.
    min: f32,
    /// Maximum gridline in world space to draw along axis.
    max: f32,
    /// 2D bounding box of pattern in world space. `x` is parallel to `axis`, `y` is perpendicular.
    pattern_bb: utils::Bounds2d<f32>,
    /// How many world position units between each gridline.
    step: f32,
    /// World position of the next gridline to generate.
    next: f32,
    /// `Axis` along which to disperse the gridlines.
    axis: Axis,
    /// Have we drawn the final gridline?
    done: bool,
}
impl GridlineIter {
    /// Creates a new `GridlineIter`.
    ///
    /// # Parameters
    ///
    /// - `min`: Minimum gridline in world space to draw along axis.
    /// - `max`: Maximum gridline in world space to draw along axis.
    /// - `axis`: Axis along which to distribute gridlines.
    /// - `step`: How many world space units between each gridline.
    /// - `pattern_bb`: 2D bounding box of pattern in world space. `x` is parallel to `axis`, `y` is perpendicular.
    fn new(min: f32, max: f32, axis: Axis, step: f32, pattern_bb: utils::Bounds2d<f32>) -> Self {
        Self {
            min: utils::maxf(min, pattern_bb.x.min),
            max: utils::minf(max, pattern_bb.x.max),
            step,
            next: utils::maxf(min, pattern_bb.x.min),
            pattern_bb,
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
        // Align gridlines to step
        if self.next == self.min {
            self.next -= self.next % self.step;
        }
        let position = self.next;
        self.next += self.step;
        self.next = self.next.min(self.max);

        let weight = match position {
            p if p == self.pattern_bb.x.min || p == self.pattern_bb.x.max => 5.0,
            p if p % (100.0 * self.step) == 0.0 => 5.0,
            p if p % (10.0 * self.step) == 0.0 => 3.0,
            _ => 1.0,
        };

        if position == self.max {
            self.done = true;
        }
        Some(Gridline {
            position,
            endpoints: utils::Bounds {
                min: self.pattern_bb.y.min,
                max: self.pattern_bb.y.max,
            },
            weight,
            axis: self.axis,
        })
    }
}

/// Generates the pattern's grid and provides it to `callback_resources`.
///
/// # Parameters
///
/// - `pattern`: The pattern whos dimensions are used to generate the grid.
/// - `camera`: The camera rendering our pattern.
/// - `frame`: Information about the current egui frame. We use it to get access to `callback_resources`.
///
/// # Returns
///
/// `GridRenderCallback` to be passed back to `egui_wgpu::Callback::new_paint_callback()`.
pub fn render(
    render_context: &mut EditorRenderContext,
    camera: &Camera,
    pattern: &Pattern,
) -> GridRendererCallback {
    // Start by filling background with a white Quad
    let mut geom = vec![];
    geom.push(Quad::from_bb(
        utils::bounds2d([[-1.0, 1.0], [-1.0, 1.0]]),
        utils::Color::WHITE,
    ));

    // Calculate grid LoD level based on camera zoom
    let grid_step = 10.0_f32.powi((((5.0 / camera.zoom).log10() + 1.0).floor() as i32).max(0));

    // Create iterators for gridlines currently in view
    let x_grids = GridlineIter::new(
        (camera.position[0] - (camera.viewport.x / (2.0 * camera.zoom))).ceil(),
        (camera.position[0] + (camera.viewport.x / (2.0 * camera.zoom))).floor(),
        Axis::X,
        grid_step,
        utils::bounds2d([
            [0.0, pattern.width as f32],
            [0.0, pattern.height as f32],
        ]),
    );
    let y_grids = GridlineIter::new(
        (camera.position[1] - (camera.viewport.y / (2.0 * camera.zoom))).ceil(),
        (camera.position[1] + (camera.viewport.y / (2.0 * camera.zoom))).floor(),
        Axis::Y,
        grid_step,
        utils::bounds2d([
            [0.0, pattern.height as f32],
            [0.0, pattern.width as f32],
        ]),
    );

    // Iterate over all gridlines in view
    for x_line in x_grids {
        geom.push(x_line.draw(camera));
    }
    for y_line in y_grids {
        geom.push(y_line.draw(camera));
    }

    render_context
        .rendered_mesh
        .append_quads(TypeId::of::<GridRendererCallback>(), geom.as_slice())
        .expect("Double dipping memory");

    GridRendererCallback {}
}

/// Callback struct used by egui to draw our rendered grid into a panel.
#[derive(Clone, Copy)]
pub struct GridRendererCallback {}
impl egui_wgpu::CallbackTrait for GridRendererCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(render_context) = &mut callback_resources.get_mut::<EditorRenderContext>() {
            let (vertex_buffer, buffer_pos, vert_bytes) = render_context
                .rendered_mesh
                .get(&TypeId::of::<Self>())
                .unwrap();

            queue.write_buffer(vertex_buffer, buffer_pos.offset_bytes, vert_bytes);
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(render_context) = callback_resources.get::<EditorRenderContext>() {
            let (vertex_buffer, buffer_pos, _vert_bytes) = render_context
                .rendered_mesh
                .get(&TypeId::of::<Self>())
                .unwrap();

            render_pass.set_pipeline(&render_context.render_pipeline);
            render_pass.set_vertex_buffer(
                0,
                vertex_buffer.slice(buffer_pos.offset_bytes..buffer_pos.len_bytes),
            );
            render_pass.draw(buffer_pos.offset_verts..buffer_pos.len_verts, 0..1);
        }
    }
}
