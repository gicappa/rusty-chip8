use crate::config::{H, VRAM, W};
use crate::cpu::Cpu;
use minifb::{Scale, Window, WindowOptions};

pub struct Gpu {
    window: Window,
    buffer: Vec<u32>,
}

impl Gpu {
    pub fn new(title: &str) -> Self {
        let mut opts = WindowOptions::default();

        opts.scale = Scale::X16;

        let mut window = Window::new(title, W, H, opts)
            .unwrap_or_else(|e| { panic!("{}", e); });
        window.set_background_color(0, 0, 0);
        window.set_target_fps(60);

        Gpu {
            window,
            buffer: vec![0u32; H * W],
        }
    }

    pub fn tick(&mut self, cpu: &mut Cpu) {
        if !self.window.is_open() {
            cpu.running = false;
        }

        if cpu.draw_flag {
            self.draw(cpu.vram);
        } else {
            self.window.update();
        }
    }

    pub fn draw(&mut self, vram: VRAM) {
        for x in 0..2048 {
            self.buffer[x] = if vram[x] { 0x00FF_FFFF } else { 0x0000_0000 };
        }

        self.window
            .update_with_buffer(&self.buffer, W, H)
            .unwrap_or_else(|e| { panic!("{}", e) })
    }
}
