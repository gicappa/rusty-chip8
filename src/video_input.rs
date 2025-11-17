use crate::config::{H, W, WXH};
use pixels::{Pixels, SurfaceTexture};
use std::sync::mpsc::Receiver;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop /*ControlFlow,*/},
    window::Window,
};

const WIDTH: u32 = W as u32;
const HEIGHT: u32 = H as u32;

type _Error = Box<dyn std::error::Error>;
type _Result<T> = Result<T, _Error>;

pub struct VideoInput {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    vram: [u8; WXH],
    rx: Option<Receiver<[u8; WXH]>>,
}

impl VideoInput {
    pub fn new(rx: Receiver<[u8; WXH]>) -> Self {
        Self {
            window: None,
            pixels: None,
            vram: [0; WXH],
            rx: Some(rx),
        }
    }
}
const BIT_ON: [u8; 4] = [0xF4, 0xDE, 0xCB, 0xFF];
const _BIT_ON_DARK: [u8; 4] = [0xB0, 0x71, 0x54, 0xFF];
const BIT_OFF: [u8; 4] = [0x3a, 0x23, 0x17, 0xFF];
const _BIT_OFF_DARK: [u8; 4] = [0x31, 0x1f, 0x13, 0xFF];
const _BIT_ON_MEDIUM: [u8; 4] = [0xA8, 0x7E, 0x62, 0xFF];

impl ApplicationHandler for VideoInput {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title("0xID8")
                .with_resizable(false)
                .with_inner_size(winit::dpi::LogicalSize::new(640.0, 320.0))
        ).unwrap();

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window_ref);
        self.pixels = Some(pixels);

        let scale_factor = window_ref.scale_factor();
        println!("Scale factor : {}", scale_factor);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Some(rx) = &self.rx {
                    while let Ok(new_vram) = rx.try_recv() {
                        self.vram = new_vram;
                    }
                }

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for (i, value) in frame.chunks_exact_mut(4).enumerate() {
                        if self.vram[i] != 0u8 {
                            value.copy_from_slice(&BIT_ON);
                        } else {
                            value.copy_from_slice(&BIT_OFF);

                        }
                    }

                    pixels.render().unwrap();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.expect("Bug - Window should exist").request_redraw();
    }
}
