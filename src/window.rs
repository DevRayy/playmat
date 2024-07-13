use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowAttributes,
};

use crate::renderer;

pub enum UserEvent {
    SetRenderer(renderer::Renderer),
}

#[derive(Default)]
pub struct WinitApplication {
    rnd: Option<renderer::Renderer>,
    event_loop_proxy: Option<EventLoopProxy<UserEvent>>,
}

impl WinitApplication {
    pub fn start(mut self) {
        let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
        self.event_loop_proxy = Some(event_loop.create_proxy());

        let _ = event_loop.run_app(&mut self);
    }
}

impl ApplicationHandler<UserEvent> for WinitApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        #[cfg(target_arch = "wasm32")]
        {
            let rnd_fut = renderer::Renderer::new(window);
            let proxy = self.event_loop_proxy.as_ref().unwrap().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let rnd = rnd_fut.await;
                let _ = proxy.send_event(UserEvent::SetRenderer(rnd));
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let rnd = pollster::block_on(renderer::Renderer::new(window));
            let _ = self
                .event_loop_proxy
                .as_ref()
                .unwrap()
                .clone()
                .send_event(UserEvent::SetRenderer(rnd));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                log::debug!("Resized to {:?}", physical_size);
            }
            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::SetRenderer(rnd) => {
                log::error!("Renderer set");
                self.rnd = Some(rnd);
            }
        }
    }
}
