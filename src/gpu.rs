use crate::config::{H, VRAM, W};
use minifb::{Scale, Window, WindowOptions};
use std::sync::mpsc::RecvTimeoutError;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub struct Gpu {
    rx: Receiver<VRAM>,
    window: Window,
}

impl Gpu {
    pub fn new(rx: Receiver<VRAM>) -> Self {
        let mut opts = WindowOptions::default();

        opts.scale = Scale::X16;

        let window = Window::new(
            "Rusty Chip-8 Emulator", W, H, opts)
            .unwrap_or_else(|e| { panic!("{}", e); });

        Gpu {
            rx,
            window,
        }
    }

    pub fn start(&mut self) {
        self.window.set_background_color(0, 0, 0);
        self.window.set_target_fps(60);

        while self.window.is_open() {
            match self.rx.recv_timeout(Duration::from_micros(1666)) {
                Ok(vram) => {
                    self.draw(&vram);
                }
                Err(RecvTimeoutError::Timeout) => {
                    self.window.update();
                }
                Err(RecvTimeoutError::Disconnected) => break, // producer is gone; exit loop
            }
        }
    }

    pub fn draw(&mut self, vram: &VRAM) {
        let buffer: Vec<u32> = vram
            .iter()
            .map(|&p| if p { 0x00FF_FFFF } else { 0x0000_0000 })
            .collect();

        self.window
            .update_with_buffer(&buffer, H, W)
            .unwrap();
    }
}
