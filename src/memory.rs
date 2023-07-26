pub struct Memory {
	buffer: Vec<u8>
}

impl Memory {
	pub fn new(program: &[u8]) -> Memory {
		let mut buffer: Vec<u8> = Vec::from(vec![0; 64]); // Interpreter memory
		buffer.extend_from_slice(program);

		Memory { 
			buffer: buffer
		}
	}

	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	pub fn read_opcode(&self, adress: usize) -> u16 {
		//debug_assert!(adress % 8 == 0); Not normal, WIP
		let idx = adress / 8;

		let mut op : u16 = 0u16;
        op += u16::from(self.buffer[idx]) << 8;
        op += u16::from(self.buffer[idx + 1]);

		op
	}

	pub fn read_bytes(&self, adress: usize, n: usize) -> &[u8] {
		//debug_assert!(adress % 8 == 0); Not normal, WIP
		let idx = adress / 8;

		&self.buffer[idx..idx+n]
	}
}