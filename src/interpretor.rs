use crate::display::Display;

pub struct Interpretor {
    program: Vec<u8>,
    screen: Display,
    registers: [u8; 16],
    i: u16,
	pc: usize
}

impl Interpretor {
	pub fn new(program: Vec<u8>) -> Interpretor {
		Interpretor {
			program: program,
			screen: Display::new(64, 32),
			registers: [0; 16],
			i: 0,
			pc: 0
		}
	}

	fn decode(&mut self, op: u16) {
		match op {
			0x0000 => {}, // Ignored
			0x00E0 => { self.screen.clear() },
			_ => println!("{} not managed", op)
		}
	}
}