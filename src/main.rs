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


    const pixel_size : usize = 16;
    let window_width = interpretor.screen_width() * pixel_size;
    let window_height = interpretor.screen_height() * pixel_size;
    let mut pixels: Vec<u32> = vec![0; window_width * window_height];
    
    let mut window = Window::new(
        "Test - ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        interpretor.cycle();

        for y in 0..window_height {
            for x in 0..window_width {
                let i = x / pixel_size;
                let j = y / pixel_size;

                if interpretor.screen_value(i, j) != 0 {
                    pixels[y * window_width + x] = 0x096096FF;
                }
                else {
                    pixels[y * window_width + x] = 0x09609680;
                }
            }
        }

        window.update_with_buffer(&pixels, window_width, window_height).unwrap();
    }
}
 