use crate::config::{H, VRAM, W};
use minifb::{Scale, Window, WindowOptions};
use std::process;

pub struct Gpu {
    window: Window,
    buffer: Vec<u32>,
}

impl Gpu {
    pub fn new() -> Self {
        let mut opts = WindowOptions::default();

        opts.scale = Scale::X16;

        let mut window = Window::new(
            "Rusty Chip-8 Emulator", W, H, opts)
            .unwrap_or_else(|e| { panic!("{}", e); });
        window.set_background_color(0, 0, 0);
        window.set_target_fps(60);

        Gpu {
            window,
            buffer: vec![0u32; H * W],
        }
    }

    pub fn clk(&mut self, vram: VRAM, draw_flag: bool) {
        if !self.window.is_open() {
            process::exit(0);
        }

        if draw_flag {
            self.draw(&vram);
        } else {
            self.window.update();
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
