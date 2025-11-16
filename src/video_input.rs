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

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub struct VideoInput {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    vram: [u8; WXH],
    rx: Option<Receiver<[u8; WXH]>>, // from CPU thread}
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
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for spot in frame.chunks_exact_mut(4) {
                        spot[0] = 0x3a; // R
                        spot[1] = 0x23; // G
                        spot[2] = 0x17; // B
                        spot[3] = 0xFF; // A
                    }

                    put_pixel(frame, 10, 5, 0xFF, 0xFF, 0xFF, 0xFF);

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

    //	#b07154	(176,113,84)
    // #f4decb	(244,222,203)
    // #a87e62	(168,126,98)
    // #3a2317	(58,35,23)
    // #311f13	(49,31,19)
}
fn put_pixel(frame: &mut [u8], x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
    // Make sure we don't go out of bounds
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    let idx = ((y * WIDTH + x) * 4) as usize;

    frame[idx] = r; // R
    frame[idx + 1] = g; // G
    frame[idx + 2] = b; // B
    frame[idx + 3] = a; // A
}
