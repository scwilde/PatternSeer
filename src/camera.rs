use crate::pattern::Pattern;
use crate::utils;
use glam::{Mat2, Vec2};

/// A camera object for panning and zooming.
#[derive(Default, Debug)]
pub struct Camera {
    /// Camera position in world space.
    pub position: Vec2,
    // TODO Replace with some sort of structure accessed like `position_bounds.x.min`
    /// The minimum and maximum position values so the user doesnt run away and lose the canvas.
    pub position_bounds: [[f32; 2]; 2],
    /// Camera viewport dimensions in logical pixels.
    pub viewport: Vec2,
    /// How many logical pixels between each single world space unit.
    pub zoom: f32,
    /// The minimum and maximum zoom values.
    pub zoom_bounds: [f32; 2],
}

impl Camera {
    pub fn new(pattern: &Pattern) -> Self{
        let pattern_width = pattern.stitched_dimensions[0] as f32;
        let pattern_height = pattern.stitched_dimensions[1] as f32;
        Self {
            position: Vec2::new(pattern_width / 2.0, pattern_height / 2.0),
            position_bounds: [[0.0, 0.0], [0.0, 0.0]],
            viewport: Vec2::new(0.0, 0.0),
            zoom: 50.0,
            zoom_bounds: [0.0, 0.0],
        }
    }

    pub fn limit_pan(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        self.position_bounds = [
            [
                ((self.viewport.x - margin) / (-2.0 * self.zoom)) + 1.0,
                (pattern.stitched_dimensions[0] as f32) + ((self.viewport.x - margin) / (2.0 * self.zoom)) - 1.0,
            ],
            [
                ((self.viewport.y - margin) / (-2.0 * self.zoom)) + 1.0,
                (pattern.stitched_dimensions[0] as f32) + ((self.viewport.y - margin) / (2.0 * self.zoom)) - 1.0,
            ],
        ];

        self.position.x = self.position.x.clamp(self.position_bounds[0][0], self.position_bounds[0][1]);
        self.position.y = self.position.y.clamp(self.position_bounds[1][0], self.position_bounds[1][1]);
    }

    pub fn limit_zoom(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        let min_zoom_x = (self.viewport.x - (margin * 2.0)) / pattern.stitched_dimensions[0] as f32;
        let min_zoom_y = (self.viewport.y - (margin * 2.0)) / pattern.stitched_dimensions[1] as f32;

        self.zoom_bounds[0] = utils::minf(min_zoom_x, min_zoom_y);
        self.zoom_bounds[1] = utils::minf(self.viewport.x, self.viewport.y) - (margin * 2.0);

        self.zoom = self.zoom.clamp(self.zoom_bounds[0], self.zoom_bounds[1]);
    }

    pub fn resize(&mut self, width: f32, height: f32, pattern: &Pattern) {
        self.viewport = Vec2::new(width, height);
    }

    /// Pans the camera through world space.
    /// Scaled with zoom so that any objects remain in the same position relative to the cursor.
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.position.x -= delta_x / self.zoom;
        self.position.y += delta_y / self.zoom;
    }

    /// Zooms the camera, increasing or decreasing the pixel size of one unit of world space.
    /// Scaled with the current zoom level so that zooming doesn't appear to slow down when zoomed 
    /// very far in or out.
    pub fn zoom(&mut self, delta_z: f32) {
        let zoom_sensitivity = 0.01;
        self.zoom += self.zoom * delta_z * zoom_sensitivity;
    }
}