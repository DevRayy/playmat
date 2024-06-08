use std::sync::Arc;

use macroquad::prelude::*;
use macroquad::ui::widgets::{Button, InputText};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};
use macroquad::window::Conf;
use tokio::sync::Mutex;
// use tokio::task::JoinHandle;

mod client;

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
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    let auth = rt.block_on(async { Arc::new(Mutex::new(client::AuthService::new().await)) });

    let mut username = String::new();
    let mut password = String::new();
    let mut clicked = false;

    let mut x = 0.0;
    let mut y = 0.0;
    let speed = 2.0;

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
                if !clicked && Button::new("Register").ui(ui) {
                    clicked = true;
                }
            });

        //draws a point that moves around the screen
        x = (x + speed) % 100.0;
        y = (y + speed) % 100.0;

        macroquad::shapes::draw_circle(x, y, 10.0, macroquad::color::RED);

        if clicked {
            let auth = auth.clone();
            let username = username.clone();
            let password = password.clone();
            rt.spawn(async move { auth.lock().await.register(username, password).await });
        }
        macroquad::text::draw_text("Hello, world!", 20.0, 20.0, 30.0, macroquad::color::BLACK);
        macroquad::window::next_frame().await
    }
}
