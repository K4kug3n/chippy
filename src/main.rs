use std::io::prelude::*;
use std::fs::File;

use chippy::interpretor::Interpretor;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut file = File::open("examples/1-chip8-logo.ch8").expect("Could not read the file {}");
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read bytes");

    let mut program: Vec<u16> = Vec::new();
    for i in (0..buffer.len()).step_by(2) {
        let mut op : u16 = 0u16;
        op += u16::from(buffer[i]) << 8;
        op += u16::from(buffer[i+1]);

        program.push(op)
    }

    println!("{:#06x?}", program);
    println!("{}", program.len());

    let mut interpretor = Interpretor::new(buffer);

    let mut pixels: Vec<u32> = vec![0; interpretor.screen_width() * interpretor.screen_height()];
    let width = interpretor.screen_width();
    let height = interpretor.screen_height();

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        interpretor.cycle();

        for y in 0..height {
            for x in 0..width {
                if interpretor.screen_value(x, y) != 0 {
                    pixels[y * width + x] = 0x096096FF;
                }
                else {
                    pixels[y * width + x] = 0x09609680;
                }
            }
        }

        window
            .update_with_buffer(&pixels, width, height)
            .unwrap();
    }
}
 