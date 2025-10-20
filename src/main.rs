mod chip8;

use crate::chip8::Chip8;

fn main() {
    println!("Chip8 Emulator Starting...");
    let mut chip8 = Chip8::new();

    chip8
        .load_rom("rom.ch8")
        .expect("File not found or not readable");

    loop {
        chip8.cycle();

        if chip8.draw_flag() {
            chip8.draw_display();
        }
    }
}
