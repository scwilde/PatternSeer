use crate::pattern::Pattern;

/// A camera object for panning and zooming.
#[derive(Default, Debug)]
pub struct Camera {
    // TODO Replace with a Vec2 from a crate, prob numix
    /// Camera position in world space.
    pub position: [f32; 2],
    // TODO Replace with a Vec2 from a crate, prob numix
    /// The minimum and maximum position values so the user doesnt run away and lose the canvas.
    pub position_bounds: [[f32; 2]; 2],
    /// Camera viewport dimensions in logical pixels.
    pub viewport: [f32; 2],
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
            position: [pattern_width / 2.0, pattern_height / 2.0],
            position_bounds: [[0.0, 0.0], [0.0, 0.0]],
            viewport: [0.0, 0.0],
            zoom: 50.0,
            zoom_bounds: [0.0, 0.0],
        }
    }

    pub fn limit_pan(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        self.position_bounds = [
            [
                ((self.viewport[0] - margin) / (-2.0 * self.zoom)) + 1.0,
                (pattern.stitched_dimensions[0] as f32) + ((self.viewport[0] - margin) / (2.0 * self.zoom)) - 1.0,
            ],
            [
                ((self.viewport[1] - margin) / (-2.0 * self.zoom)) + 1.0,
                (pattern.stitched_dimensions[0] as f32) + ((self.viewport[1] - margin) / (2.0 * self.zoom)) - 1.0,
            ],
        ];

        self.position[0] = self.position[0].min(self.position_bounds[0][1]).max(self.position_bounds[0][0]);
        self.position[1] = self.position[1].min(self.position_bounds[1][1]).max(self.position_bounds[1][0]);
    }
    pub fn limit_zoom(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        let min_zoom_x = (self.viewport[0] - (margin * 2.0)) / pattern.stitched_dimensions[0] as f32;
        let min_zoom_y = (self.viewport[1] - (margin * 2.0)) / pattern.stitched_dimensions[1] as f32;

        self.zoom_bounds[0] = min_zoom_x.min(min_zoom_y);
        self.zoom_bounds[1] = self.viewport[0].min(self.viewport[1]) - (margin * 2.0);

        self.zoom = self.zoom.min(self.zoom_bounds[1]).max(self.zoom_bounds[0]);
    }

    pub fn resize(&mut self, width: f32, height: f32, pattern: &Pattern) {
        self.viewport = [width, height];
    }

    /// Pans the camera through world space.
    /// Scaled with zoom so that any world-space objects remain in the same position relative to the cursor.
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.position[0] -= delta_x / self.zoom;
        self.position[1] += delta_y / self.zoom;
    }

    /// Zooms the camera, increasing or decreasing the pixel size of one unit of world space.
    /// Scaled with the current zoom level so that zooming doesn't appear to slow down when zoomed 
    /// very far in or out.
    pub fn zoom(&mut self, delta_z: f32) {
        let zoom_sensitivity = 0.01;
        self.zoom += self.zoom * delta_z * zoom_sensitivity;
    }
}