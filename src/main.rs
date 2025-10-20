use minifb::{Key, Window, WindowOptions};

fn main() {
    const MEMORY_SIZE: usize = 4096;
    const WIDTH: usize = 64;
    const HEIGHT: usize = 32;
    const SCALE: usize = 10;
    let mut buffer: Vec<u32> = vec![1; WIDTH * HEIGHT * SCALE * SCALE];

    struct Chip8 {
        memory: [u8; MEMORY_SIZE],
        v: [u8; 16],
        i: u16,
        pc: u16,
        sp: u8,
        stack: [u16; 16],
        delay_timer: u8,
        sound_timer: u8,
        keypad: [u8; 16],
        display: [u8; WIDTH * HEIGHT],
    }

    println!("Chip8 Emulator Starting...");
    println!("Memory available: {}", MEMORY_SIZE);
    // setup_chip8();
    // setup_input();
    // initialize_chip8();
    println!("Current Opcode: {:04X}", opcode);

}
