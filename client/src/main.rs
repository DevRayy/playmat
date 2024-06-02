use macroquad::prelude::*;
use macroquad::ui::widgets::{Button, InputText};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};
use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Playmat".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut username = String::new();
    let mut password = String::new();
    loop {
        macroquad::window::clear_background(macroquad::color::BLACK);
        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
            .label("Register")
            .titlebar(false)
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                InputText::new(hash!())
                    .label("Username")
                    .ui(ui, &mut username);
                InputText::new(hash!())
                    .label("Password")
                    .password(true)
                    .ui(ui, &mut password);
                if Button::new("Register").ui(ui) {
                    println!("Registering...");
                }
            });
        macroquad::text::draw_text("Hello, world!", 20.0, 20.0, 30.0, macroquad::color::BLACK);
        macroquad::window::next_frame().await
    }
}
