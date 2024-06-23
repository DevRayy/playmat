use macroquad::prelude::*;
use macroquad::ui::hash;
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

pub struct FPSCounter {
    update_rate: f64,
    last_update: f64,
    fps_count: i32,
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            update_rate: 0.5,
            last_update: 0.0,
            fps_count: 0,
        }
    }

    pub fn draw(&mut self) {
        if macroquad::time::get_time() - self.last_update > self.update_rate {
            self.last_update = macroquad::time::get_time();
            self.fps_count = macroquad::time::get_fps();
        }

        macroquad::text::draw_text(
            &format!("FPS: {}", self.fps_count),
            20.0,
            50.0,
            50.0,
            macroquad::color::WHITE,
        );
    }
}

pub struct RegisterWindow {
    window: macroquad::ui::widgets::Window,
    username: String,
    password: String,
}

impl RegisterWindow {
    pub fn new() -> RegisterWindow {
        RegisterWindow {
            window: macroquad::ui::widgets::Window::new(
                hash!(),
                vec2(400., 200.),
                vec2(320., 400.),
            )
            .label("Register")
            .titlebar(false)
            .movable(false),
            username: String::new(),
            password: String::new(),
        }
    }

    pub fn draw(&mut self) {
        self.window.clone().ui(&mut macroquad::ui::root_ui(), |ui| {
            macroquad::ui::widgets::InputText::new(hash!())
                .label("Username")
                .ui(ui, &mut self.username);
            macroquad::ui::widgets::InputText::new(hash!())
                .label("Password")
                .password(true)
                .ui(ui, &mut self.password);
            macroquad::ui::widgets::Button::new("Register").ui(ui);
        });
    }
}
