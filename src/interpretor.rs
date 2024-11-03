use crate::display::Display;

pub struct Interpretor {
    program: Vec<u16>,
    screen: Display,
    registers: [u8; 16],
    l: u16
}

impl Interpretor {
	pub fn new(program: Vec<u16>) -> Interpretor {
		Interpretor {
			program: program,
			screen: Display::new(64, 32),
			registers: [0; 16],
			l: 0
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