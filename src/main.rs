use std::io::prelude::*;
use std::fs::File;

use chippy::interpretor::Interpretor;

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

    interpretor.run();
}
 