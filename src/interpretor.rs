use crate::display::Display;
use crate::memory::Memory;

pub struct Interpretor {
    memory: Memory,
    screen: Display,
    registers: [u8; 16],
    i: u16,
	pc: usize
}

impl Interpretor {
	pub fn new(program: Vec<u8>) -> Interpretor {
		Interpretor {
			memory: Memory::new(&program),
			screen: Display::new(64, 32),
			registers: [0; 16],
			i: 0,
			pc: 0x200
		}
	}

	pub fn screen_width(&self) -> usize {
		usize::from(self.screen.width())
	}

	pub fn screen_height(&self) -> usize {
		usize::from(self.screen.height())
	}

	pub fn screen_value(&self, x: usize, y: usize) -> u8 {
		self.screen.get(x, y)
	}

	pub fn cycle(&mut self) {
		let op : u16 = self.memory.read_opcode(self.pc);
		self.decode(op);
		self.pc += 16;
	}

	fn decode(&mut self, op: u16) {
		let first_byte = op & 0xF000;

		match first_byte {
			0x0000 => self.decode_0(op),
			0x1000 => { 
				// 0x1NNN
				self.pc = usize::from(op & 0x0FFF);
			},
			0x2000 => { println!("Need {:#06x?} opcode", op) },
			0x3000 => { println!("Need {:#06x?} opcode", op) },
			0x4000 => { println!("Need {:#06x?} opcode", op) },
			0x5000 => { println!("Need {:#06x?} opcode", op) },
			0x6000 => {
				// 0x6XNN
				let index = usize::from((op & 0x0F00) >> 8);
				let value = u8::try_from(op & 0x00FF).unwrap();
				self.registers[index] = value;
			},
			0x7000 => { println!("Need {:#06x?} opcode", op) },
			0x8000 => { println!("Need {:#06x?} opcode", op) },
			0x9000 => { println!("Need {:#06x?} opcode", op) },
			0xA000 => {
				// 0xANNN
				let value = op & 0x0FFF;
				self.i = value;
			},
			0xB000 => { println!("Need {:#06x?} opcode", op) },
			0xC000 => { println!("Need {:#06x?} opcode", op) },
			0xD000 => {
				// 0xDXYN
				let n = usize::from(op & 0x000F);
				let sprite = self.memory.read_bytes(usize::from(self.i), n);
				let vx_idx = usize::from((op & 0x0F00) >> 8);
				let vy_idx = usize::from((op & 0x00F0) >> 4);

				let x = self.registers[vx_idx];
				let y = self.registers[vy_idx];
				self.screen.draw(x, y, sprite);
			},
			0xE000 => { println!("Need {:#06x?} opcode", op) },
			0xF000 => { println!("Need {:#06x?} opcode", op) },
			_ => println!("{:#06x?} not managed", op)
		}
	}

	fn decode_0(&mut self, op: u16) {
		match op {
			0x0000 => {}, // Ignored
			0x00E0 => { self.screen.clear() },
			// 0x00EE
			_ => println!("{:#06x?} not managed", op)
		}
	}
}