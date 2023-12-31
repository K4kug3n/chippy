use std::io::prelude::*;
use std::fs::File;
use std::time::Duration;

use chippy::interpretor::Interpretor;

use minifb::{Key, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {
    let mut file = File::open("examples/PONG.ch8").expect("Could not read the file {}");
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read bytes");

    let mut interpretor = Interpretor::new(buffer);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    const PIXEL_SIZE : usize = 16;
    let window_width = usize::from(interpretor.screen.width()) * PIXEL_SIZE;
    let window_height = usize::from(interpretor.screen.height()) * PIXEL_SIZE;
    let mut pixels: Vec<u32> = vec![0; window_width * window_height];
    
    let mut window = Window::new(
        "Test - ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(None);

    while window.is_open() && !window.is_key_down(Key::Escape) && !interpretor.is_finished() {
        
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

        if interpretor.is_beeping() {
            let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.02)).amplify(0.20);

            sink.append(source);
        }

        if interpretor.has_drawn() {
            for y in 0..usize::from(interpretor.screen.height()) {
                let window_y = y * PIXEL_SIZE;
                for x in 0..usize::from(interpretor.screen.width()) {
                    let window_x = x * PIXEL_SIZE;
    
                    let color = if interpretor.screen.get(x, y) != 0 { 0x096096FF } else { 0x09609680 };
    
                    for j in 0..PIXEL_SIZE {
                        for i in 0..PIXEL_SIZE {
                            pixels[(window_y + j) * window_width + window_x + i] = color;
                        }
                    }
                }
            }
        }

        window.update_with_buffer(&pixels, window_width, window_height).unwrap();
        
    }
}
 