use crate::display::Display;
use crate::memory::Memory;

pub struct Interpretor {
    memory: Memory,
    screen: Display,
    registers: [u8; 16],
	stack: Vec<u16>,
    i: u16,
	pc: usize
}

impl Interpretor {
	pub fn new(program: Vec<u8>) -> Interpretor {
		Interpretor {
			memory: Memory::new(&program),
			screen: Display::new(64, 32),
			registers: [0; 16],
			stack: Vec::new(),
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
		self.pc += 2;
	}

	fn decode(&mut self, op: u16) {
		let first_byte = op & 0xF000;

		match first_byte {
			0x0000 => self.decode_0(op),
			0x1000 => { 
				// 0x1NNN
				self.pc = usize::from(op & 0x0FFF);
				self.pc -= 2; // Prepare increment
			},
			0x2000 => {
				// 0x2NNN
				self.stack.push(u16::try_from(self.pc).unwrap());
				
				let adress = op & 0x0FFF;
				self.pc = usize::from(adress);
			},
			0x3000 => { 
				// 0x3XNN
				let index = usize::from((op & 0x0F00) >> 8);
				let value = u8::try_from(op & 0x00FF).unwrap();
				if self.registers[index] == value {
					self.pc += 2;
				}
			},
			0x4000 => {
				// 0x4XNN
				let index = usize::from((op & 0x0F00) >> 8);
				let value = u8::try_from(op & 0x00FF).unwrap();
				if self.registers[index] != value {
					self.pc += 2;
				}
			},
			0x5000 => {
				// 0x5XY0
				let vx = usize::from((op & 0x0F00) >> 8);
				let vy = usize::from((op & 0x00F0) >> 4);
				if self.registers[vx] == self.registers[vy] {
					self.pc += 2;
				}
			},
			0x6000 => {
				// 0x6XNN
				let index = usize::from((op & 0x0F00) >> 8);
				let value = u8::try_from(op & 0x00FF).unwrap();
				self.registers[index] = value;
			},
			0x7000 => { 
				// 0x7XNN
				let index = usize::from((op & 0x0F00) >> 8);
				let value = u8::try_from(op & 0x00FF).unwrap();

				self.registers[index] = u8::wrapping_add(self.registers[index], value);
			},
			0x8000 => self.decode_8(op),
			0x9000 => {
				// 0x9XY0
				let vx = usize::from((op & 0x0F00) >> 8);
				let vy = usize::from((op & 0x00F0) >> 4);
				if self.registers[vx] != self.registers[vy] {
					self.pc += 2;
				}
			},
			0xA000 => {
				// 0xANNN
				self.i = op & 0x0FFF;
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
			0xF000 => self.decode_f(op),
			_ => println!("{:#06x?} not managed", op)
		}
	}

	fn decode_0(&mut self, op: u16) {
		match op {
			0x0000 => {}, // Ignored
			0x00E0 => { self.screen.clear() },
			0x00EE => {
				let adress = self.stack.pop().unwrap();
				self.pc = usize::from(adress);
			},
			_ => println!("{:#06x?} not managed", op)
		}
	}

	fn decode_8(&mut self, op: u16) {
		let last_byte = op & 0x000F;
		let vx = usize::from((op & 0x0F00) >> 8);
		let vy = usize::from((op & 0x00F0) >> 4);
		match last_byte {
			0x0 => {
				self.registers[vx] = self.registers[vy];
			},
			0x1 => {
				self.registers[vx] |= self.registers[vy];
			},
			0x2 => {
				self.registers[vx] &= self.registers[vy];
			},
			0x3 => {
				self.registers[vx] ^= self.registers[vy];
			},
			0x4 => {
				let res = u8::overflowing_add(self.registers[vx], self.registers[vy]);
				self.registers[vx] = res.0;

				if res.1 {
					self.registers[0xF] = 1;
				}
				else {
					self.registers[0xF] = 0;
				}
			},
			0x5 => {
				let res = u8::overflowing_sub(self.registers[vx], self.registers[vy]);
				self.registers[vx] = res.0;
				
				if res.1 {
					self.registers[0xF] = 0;
				}
				else {
					self.registers[0xF] = 1;
				}
			},
			0x6 => {
				self.registers[0xF] = self.registers[vx] & 0x0001; // Least significant bit
				self.registers[vx] >>= 1;
			},
			0x7 => {
				let res = u8::overflowing_sub(self.registers[vy], self.registers[vx]);
				self.registers[vx] = res.0;
				
				if res.1 {
					self.registers[0xF] = 0;
				}
				else {
					self.registers[0xF] = 1;
				}
			},
			0xE => {
				self.registers[0xF] = (self.registers[vx] & 0x80) >> 7; // Most significant bit
				self.registers[vx] = (self.registers[vx] & 0x7F) << 1;
			},
			_ => println!("{:#06x?} not managed", op)
		}
	}

	fn decode_f(&mut self, op: u16) {
		let last_2_bytes = op & 0x00FF;
		let vx = usize::from((op & 0x0F00) >> 8);
		match last_2_bytes {
			0x1E => { 
				self.i += u16::from(self.registers[vx])
			},
			0x33 => {
				let value = self.registers[vx];

				let hundreds = value / 100;
				let decimals = (value - hundreds * 100) / 10;
				let ones = value % 10;

				self.memory.write(usize::from(self.i), hundreds);
				self.memory.write(usize::from(self.i) + 1, decimals);
				self.memory.write(usize::from(self.i) + 2, ones);
			},
			0x55 => {
				for i in 0..=vx {
					self.memory.write(usize::from(self.i) + i, self.registers[i]);
				}
			},
			0x65 => {
				for i in 0..=vx {
					self.registers[i] = self.memory.read(usize::from(self.i) + i);
				}
			},
			_ => println!("{:#06x?} not managed", op)
		}
	}
}