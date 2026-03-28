pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
    pub zoom: i32,
    pub panning: bool,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: width,
            height: height,
            zoom: 20,
            panning: false
        }
    }

    pub fn pan(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
        println!("Camera panned by [{}, {}]  to [{}, {}]", delta_x, delta_y, self.x, self.y);
    }
}