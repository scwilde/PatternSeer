/// A camera object for panning and zooming.
#[derive(Default, Debug)]
pub struct Camera {
    // TODO Replace with a Vec2 from a crate, prob numix
    /// Camera position in world space.
    pub position: [f32; 2],
    // TODO Replace with a Vec2 from a crate, prob numix
    /// Camera viewport dimensions in logical pixels.
    pub viewport: [f32; 2],
    /// How many logical pixels between each single world space unit.
    pub zoom: f32,
}

impl Camera {
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