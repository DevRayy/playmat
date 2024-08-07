#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod renderer;
mod texture;
mod window;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let app = window::WinitApplication::default();
    app.start();
}
