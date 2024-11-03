use std::io::prelude::*;
use std::fs::File;

use chippy::interpretor::Interpretor;

fn main() {
    let mut file = File::open("examples/1-chip8-logo.ch8").expect("Could not read the file {}");
    
    let mut program = Vec::new();
    file.read_to_end(&mut program).expect("Could not read bytes");

    println!("{:#06x?}", program);


    let interpretor = Interpretor::new(program);
}
 