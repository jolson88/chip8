#[macro_use]
extern crate enum_primitive_derive;
extern crate minifb;
extern crate num_traits;
extern crate rand;

mod chip8;
mod opcode;
use chip8::Chip8;
use minifb::{Key, Scale, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load program from file
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Create emulator
    let mut chip8 = Chip8::default();
    chip8.load_program(&data[..]);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0x99_00_00;
        }

        chip8.tick()?;
        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
