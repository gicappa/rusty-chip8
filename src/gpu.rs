use crate::config::{H, VRAM, W};

use minifb::{Scale, Window, WindowOptions, Key};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread::sleep;
use std::time::Duration;

pub struct Gpu {
    rx: Receiver<VRAM>,
    key_tx: Sender<u8>,
    window: Window,
    buffer: Vec<u32>,
}

impl Gpu {
    pub fn new(rx: Receiver<VRAM>, key_tx: Sender<u8>) -> Self {
        let mut opts = WindowOptions::default();

        opts.scale = Scale::X16;

        let window = Window::new(
            "Rusty Chip-8 Emulator", W, H, opts)
            .unwrap_or_else(|e| { panic!("{}", e); });

        Gpu {
            rx,
            key_tx,
            window,
            buffer: vec![0u32; H * W],
        }
    }

    pub fn start(&mut self) {
        self.window.set_background_color(0, 0, 0);
        self.window.set_target_fps(60);

        // CHIP-8 key mapping
        let key_map = [
            Key::Key1, Key::Key2, Key::Key3, Key::Key4,
            Key::Q, Key::W, Key::E, Key::R,
            Key::A, Key::S, Key::D, Key::F,
            Key::Z, Key::X, Key::C, Key::V,
        ];

        while self.window.is_open() {
            // Check for key presses and send them
            for (index, &key) in key_map.iter().enumerate() {
                if self.window.is_key_pressed(key, minifb::KeyRepeat::No) {
                    let _ = self.key_tx.send(index as u8);
                }
            }

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

    pub fn draw(&mut self, vram: &VRAM) {
        for x in 0..2048 {
            self.buffer[x] = if vram[x] { 0x00FF_FFFF } else { 0x0000_0000 };
        }

        self.window
            .update_with_buffer(&self.buffer, W, H)
            .unwrap_or_else(|e| { panic!("{}", e) })
    }
}
