use std::io::prelude::*;
use std::fs::File;

struct Interpretor {
    program : Vec<u8>,
    registers : [u8; 16],
    l : u16
}

fn decode(op: u8) {
    match op {
        0x0000 => {}, // Ignored
        0x00E0 => {},
        _ => println!("{} not managed", op)
    }
}

fn main() {
    let mut file = File::open("examples/1-chip8-logo.ch8").expect("Could not read the file {}");
    
    let mut program = Vec::new();
    file.read_to_end(&mut program).expect("Could not read bytes");

    println!("{:#06x?}", program);
}
 