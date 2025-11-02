use minifb::{Scale, Window, WindowOptions};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

pub const W: usize = 64;
pub const H: usize = 32;
pub const PIXELS: usize = W * H;

pub struct GPU {
    rx: Receiver<[u8; PIXELS]>,
    window: Window,
}

impl GPU {

    pub fn new(rx: Receiver<[u8; PIXELS]>) -> Self {
        let mut opts = WindowOptions::default();

        opts.scale = Scale::X16;

        let window = Window::new(
            "Rusty Chip-8 Emulator", W, H, opts)
            .unwrap_or_else(|e| { panic!("{}", e); });

        GPU {
            rx,
            window,
        }
    }

    pub fn start(&mut self) {
        self.window.set_background_color(0, 0, 0);

        self.window
            .limit_update_rate(Some(Duration::from_millis(4)));

        while self.window.is_open() {
            match self.rx.try_recv() {
                Ok(vram) => {
                    self.draw(&vram);
                }
                Err(TryRecvError::Empty) => {
                    self.window.update();
                }
                Err(TryRecvError::Disconnected) => break, // producer is gone; exit loop
            }
        }
    }

    pub fn draw(&mut self, vram: &[u8; PIXELS]) {
        let buffer: Vec<u32> = vram
            .iter()
            .map(|&pix| if pix != 0 { 0x00FF_FFFF } else { 0x0000_0000 })
            .collect();

        self.window
            .update_with_buffer(&buffer, H, W)
            .unwrap();
    }
}
