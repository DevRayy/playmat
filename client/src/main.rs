use std::sync::Arc;

use macroquad::prelude::*;
use macroquad::ui::widgets::{Button, InputText, Label};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};
use macroquad::window::Conf;
use tokio::sync::Mutex;
// use tokio::task::JoinHandle;

mod client;
mod ui;

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

    let loader = ui::Loader::new(ui::Position { x: 100.0, y: 100.0 });
    let mut fps_counter = ui::FPSCounter::new();
    let mut window = ui::RegisterWindow::new();

    loop {
        macroquad::window::clear_background(macroquad::color::BLACK);
        window.draw();
        fps_counter.draw();
        loader.draw();

        // if clicked {
        //     let auth = auth.clone();
        //     let username = username.clone();
        //     let password = password.clone();
        //     rt.spawn(async move {
        //         let _ = auth.lock().await.register(username, password).await;
        //     });
        // }
        macroquad::text::draw_text("Hello, world!", 20.0, 20.0, 30.0, macroquad::color::BLACK);
        macroquad::window::next_frame().await
    }
}
