pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 20,
        }
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.x -= delta_x;
        self.y -= delta_y;
        println!("Camera panned by [{}, {}]  to [{}, {}]", delta_x, delta_y, self.x, self.y);
    }
}