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

    let auth = rt.block_on(async {
        Arc::new(Mutex::new(client::AuthService::new().await))
    });
    let mut username = String::new();
    let mut password = String::new();
    let mut clicked = false;

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
            if clicked == true {
                rt.block_on(async{
                    let mut auth = auth.lock().await;
                    auth.register(username.clone(), password.clone()).await.unwrap();
                    clicked = false;
                });
            }
        macroquad::text::draw_text("Hello, world!", 20.0, 20.0, 30.0, macroquad::color::BLACK);
        macroquad::window::next_frame().await
    }
}