use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes},
};

use crate::renderer;

pub enum UserEvent {
    SetRenderer(renderer::Renderer),
}

#[derive(Default)]
pub struct WinitApplication {
    rnd: Option<renderer::Renderer>,
    window: Option<Arc<Window>>,
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
        let size = winit::dpi::PhysicalSize::new(1280, 720);
        let window = event_loop
            .create_window(WindowAttributes::default()
            .with_inner_size(size).with_resizable(false)).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
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

        self.window = Some(Arc::new(window));

        #[cfg(target_arch = "wasm32")]
        {
            let rnd_fut = renderer::Renderer::new(self.window.as_ref().unwrap().clone());
            let proxy = self.event_loop_proxy.as_ref().unwrap().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let rnd = rnd_fut.await;
                let _ = proxy.send_event(UserEvent::SetRenderer(rnd));
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let rnd = pollster::block_on(renderer::Renderer::new(
                self.window.as_ref().unwrap().clone(),
            ));
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
                log::error!("{:?}", physical_size);
                if self.rnd.is_none() {
                    return;
                }

                #[cfg(target_arch = "wasm32")]
                let physical_size = winit::dpi::PhysicalSize::new(1280, 720); //workaround for mysterious events on wasm

                self.rnd.as_mut().unwrap().resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                if self.rnd.is_none() {
                    return;
                }

                match self.rnd.as_mut().unwrap().render() {
                    Ok(_) => (),
                    Err(wgpu::SurfaceError::Lost) => {
                        self.rnd
                            .as_mut()
                            .unwrap()
                            .resize(self.window.as_ref().unwrap().inner_size());
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("Out of memory");
                    }
                    Err(e) => log::error!("{:?}", e),
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::SetRenderer(rnd) => {
                self.rnd = Some(rnd);
            }
        }
    }
}
