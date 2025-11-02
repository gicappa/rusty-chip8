use minifb::{Key, Scale, Window, WindowOptions};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

pub struct GPU<const W: usize, const H: usize> {
    rx: Receiver<[u8; 2048]>,
    window: Window,
}

impl<const W: usize, const H: usize> GPU<W, H> {
    pub fn new(rx: Receiver<[u8; 2048]>) -> Self {
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

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            match self.rx.try_recv() {
                Ok(vram) => {
                    self.draw(&vram);
                }
                Err(TryRecvError::Empty) => {
                    self.window.update();
                }
                Err(TryRecvError::Disconnected) => break, // producer is gone; exit loop
            }

            self.window.update()
        }
    }

    pub fn draw(&mut self, vram: &[u8; 2048]) {
        let buffer: Vec<u32> = vram
            .iter()
            .map(|&pix| if pix != 0 { 0x00FFFFFF } else { 0x00000000 })
            .collect();

        self.window
            .update_with_buffer(&buffer, H, W)
            .unwrap();
    }
}
