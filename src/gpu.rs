use crate::config::{H, VRAM, W};
use minifb::{Scale, Window, WindowOptions};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread::sleep;
use std::time::Duration;

pub struct Gpu {
    rx: Receiver<VRAM>,
    window: Window,
    buffer: Vec<u32>,
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
            buffer: vec![0u32; H * W],
        }
    }

    pub fn start(&mut self) {
        self.window.set_background_color(0, 0, 0);
        self.window.set_target_fps(60);

        while self.window.is_open() {
            while self.window.is_open() {
                match self.rx.recv_timeout(Duration::from_micros(1666)) {
                    Ok(vram) => {
                        self.draw(&vram);
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        self.window.update();
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                }
                sleep(Duration::from_millis(10));
            }
        }
    }

    pub fn draw(&mut self, vram: &VRAM) {
        for x in 0..2048 {
            self.buffer[x] = if vram[x] { 0x00FF_FFFF } else { 0x0000_0000 };
        }

        self.window
            .update_with_buffer(&self.buffer, W, H)
            .unwrap_or_else(|e| { panic!("{}", e) })
    }
}
