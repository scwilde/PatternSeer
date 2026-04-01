pub struct Camera {
    /// Camera position in world space
    pub position: [f32; 2],
    /// Camera dimensions in logical pixels
    pub viewport: [f32; 2],
    /// How many pixels between each single world space unit
    pub zoom: f32,
}

impl Camera {
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.position[0] -= delta_x / self.zoom;
        self.position[1] += delta_y / self.zoom;
    }

    pub fn zoom(&mut self, delta_z: f32) {
        let zoom_sensitivity = 0.01;
        self.zoom += self.zoom * delta_z * zoom_sensitivity;
    }
}