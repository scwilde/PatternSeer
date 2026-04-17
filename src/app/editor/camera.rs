use crate::pattern::Pattern;
use crate::utils;
use glam::Vec2;

/// A camera object for panning and zooming.
#[derive(Default, Debug)]
pub struct Camera {
    /// Camera position in world space.
    pub position: Vec2,
    /// Camera viewport dimensions in logical pixels.
    pub viewport: Vec2,
    /// How many logical pixels between each single world space unit.
    pub zoom: f32,
}

impl Camera {
    /// Calculate where the camera is allowed to be in space without losing the pattern.
    /// If the camera's position is outside those bounds, clamp it.
    ///
    /// # Parameters
    ///
    /// - `pattern`: Pattern used to determine where the camera is allowed to pan.
    pub fn limit_pan(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        let position_bounds = utils::Bounds2d {
            x: utils::Bounds {
                min: ((self.viewport.x - margin) / (-2.0 * self.zoom)) + 1.0,
                max: (pattern.metadata.width as f32) + ((self.viewport.x - margin) / (2.0 * self.zoom)) - 1.0,
            },
            y: utils::Bounds {
                min: ((self.viewport.y - margin) / (-2.0 * self.zoom)) + 1.0,
                max: (pattern.metadata.height as f32) + ((self.viewport.y - margin) / (2.0 * self.zoom)) - 1.0,
            },
        };

        self.position.x = self.position.x.clamp(position_bounds.x.min, position_bounds.x.max);
        self.position.y = self.position.y.clamp(position_bounds.y.min, position_bounds.y.max);
    }

    /// Calculate how far in/out the camera is allowed to zoom and clamp it to those values.
    ///
    /// # Parameters
    ///
    /// - `pattern`: `Pattern` used to determine how far the camera can zoom in or out based on its dimensions.
    pub fn limit_zoom(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        let min_zoom_x = (self.viewport.x - (margin * 2.0)) / pattern.metadata.width as f32;
        let min_zoom_y = (self.viewport.y - (margin * 2.0)) / pattern.metadata.height as f32;
        let zoom_bounds = utils::Bounds {
            min: utils::minf(min_zoom_x, min_zoom_y),
            max: utils::minf(self.viewport.x, self.viewport.y) - (margin * 2.0),
        };

        self.zoom = self.zoom.clamp(zoom_bounds.min, zoom_bounds.max);
    }

    /// Resize the camera's viewport to the specified dimensions.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.viewport.x = width;
        self.viewport.y = height;
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

    pub fn center(&mut self, pattern: &Pattern) {
        let margin = 50.0;
        let min_zoom_x = (self.viewport.x - (margin * 2.0)) / pattern.metadata.width as f32;
        let min_zoom_y = (self.viewport.y - (margin * 2.0)) / pattern.metadata.height as f32;
        self.zoom = utils::minf(min_zoom_x, min_zoom_y);

        self.position.x = (pattern.metadata.width as f32) / 2.0;
        self.position.y = (pattern.metadata.height as f32) / 2.0;
    }
}
