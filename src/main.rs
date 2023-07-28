use std::io::prelude::*;
use std::fs::File;

use chippy::interpretor::Interpretor;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut file = File::open("examples/2-ibm-logo.ch8").expect("Could not read the file {}");
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read bytes");

    let mut interpretor = Interpretor::new(buffer);

    const PIXEL_SIZE : usize = 16;
    let window_width = interpretor.screen_width() * PIXEL_SIZE;
    let window_height = interpretor.screen_height() * PIXEL_SIZE;
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
        
        window.get_keys_pressed(minifb::KeyRepeat::No).iter().for_each(|key|
            match key {
                Key::NumPad0 => interpretor.set_pressed(0x0),
                Key::NumPad1 => interpretor.set_pressed(0x1),
                Key::NumPad2 => interpretor.set_pressed(0x2),
                Key::NumPad3 => interpretor.set_pressed(0x3),
                Key::NumPad4 => interpretor.set_pressed(0x4),
                Key::NumPad5 => interpretor.set_pressed(0x5),
                Key::NumPad6 => interpretor.set_pressed(0x6),
                Key::NumPad7 => interpretor.set_pressed(0x7),
                Key::NumPad8 => interpretor.set_pressed(0x8),
                Key::NumPad9 => interpretor.set_pressed(0x9),
                Key::A => interpretor.set_pressed(0xA),
                Key::B => interpretor.set_pressed(0xB),
                Key::C => interpretor.set_pressed(0xC),
                Key::D => interpretor.set_pressed(0xD),
                Key::E => interpretor.set_pressed(0xE),
                Key::F => interpretor.set_pressed(0xF),
                _ => {}
            }
        );

        window.get_keys_released().iter().for_each(|key|
            match key {
                Key::NumPad0 => interpretor.set_released(0x0),
                Key::NumPad1 => interpretor.set_released(0x1),
                Key::NumPad2 => interpretor.set_released(0x2),
                Key::NumPad3 => interpretor.set_released(0x3),
                Key::NumPad4 => interpretor.set_released(0x4),
                Key::NumPad5 => interpretor.set_released(0x5),
                Key::NumPad6 => interpretor.set_released(0x6),
                Key::NumPad7 => interpretor.set_released(0x7),
                Key::NumPad8 => interpretor.set_released(0x8),
                Key::NumPad9 => interpretor.set_released(0x9),
                Key::A => interpretor.set_released(0xA),
                Key::B => interpretor.set_released(0xB),
                Key::C => interpretor.set_released(0xC),
                Key::D => interpretor.set_released(0xD),
                Key::E => interpretor.set_released(0xE),
                Key::F => interpretor.set_released(0xF),
                _ => {}
            }
        );

        interpretor.cycle();

        for y in 0..window_height {
            for x in 0..window_width {
                let i = x / PIXEL_SIZE;
                let j = y / PIXEL_SIZE;

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
 