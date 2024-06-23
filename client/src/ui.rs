use std::f32::consts::PI;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Loader {
    pos: Position,
    spread: f32,
    size: f32,
    speed: f32,
    count: u8,
}

impl Loader {
    pub fn new(pos: Position) -> Loader {
        Loader {
            pos,
            spread: 50.0,
            size: 25.0,
            speed: 5.0,
            count: 3,
        }
    }

    pub fn draw(&self) {
        for i in 0..self.count {
            let phase = 2.0 * PI * (i as f32) / (self.count as f32);
            macroquad::shapes::draw_circle(
                self.pos.x
                    + self.spread
                        * f32::cos(macroquad::time::get_time() as f32 * self.speed + phase),
                self.pos.y
                    + self.spread
                        * f32::sin(macroquad::time::get_time() as f32 * self.speed + phase),
                self.size,
                macroquad::color::RED,
            );
        }
    }
}
