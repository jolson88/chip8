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
use std::time::Instant;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const PIXEL_SIZE: usize = 10;
const CLOCK_SPEED: u32 = 600;
/// The ideal frame duration in nanoseconds at the desired CLOCK_SPEED
const FRAME_DURATION_NS: u128 = 1_000_000_000 / CLOCK_SPEED as u128;

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

    let mut last_update = Instant::now();
    let mut elapsed_ns: u128 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..(HEIGHT / PIXEL_SIZE) {
            for x in 0..(WIDTH / PIXEL_SIZE) {
                let pixel = chip8.get_pixel(x, y);
                // Fill in all the pixels necessary (we are effectively "zoom in" via PIXEL_SIZE)
                for j in 0..PIXEL_SIZE {
                    for i in 0..PIXEL_SIZE {
                        let dest_x = x * PIXEL_SIZE + i;
                        let dest_y = y * PIXEL_SIZE + j;
                        buffer[dest_y * WIDTH + dest_x] = 0xFF_FF_FF * u32::from(pixel);
                    }
                }
            }
        }

        // Run Chip-8 emulator at CLOCK_SPEED (60hz by default)
        // We do this by keeping a timer (elapsed_ns) of how many nanoseconds have elapsed.
        // Once enough nanoseconds have elapsed for a "tick", we run the tick. Any leftover
        // nanoseconds are carried over so that even if the loop timing is inconsistent, the
        // clock rate will largely remain fairly stable.
        let now = Instant::now();
        elapsed_ns += now.duration_since(last_update).as_nanos();
        let tick_count = elapsed_ns / FRAME_DURATION_NS as u128;
        for _ in 0..tick_count {
            chip8.tick()?;
        }

        window.update_with_buffer(&buffer)?;
        elapsed_ns %= FRAME_DURATION_NS;
        last_update = now; 
    }

    Ok(())
}
