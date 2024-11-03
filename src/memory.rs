pub struct Memory {
	buffer: Vec<u8>
}

impl Memory {
	pub fn new(program: &[u8]) -> Memory {
		let mut buffer: Vec<u8> = vec![0; 512]; // Interpreter memory

		buffer[..80].copy_from_slice(&[
			0xF0, 0x90, 0x90, 0x90, 0xF0,       // 0
			0x20, 0x60, 0x20, 0x20, 0x70,		// 1
			0xF0, 0x10, 0xF0, 0x80, 0xF0,		// 2
			0xF0, 0x10, 0xF0, 0x10, 0xF0,		// 3
			0x90, 0x90, 0xF0, 0x10, 0x10,		// 4
			0xF0, 0x80, 0xF0, 0x10, 0xF0,		// 5
			0xF0, 0x80, 0xF0, 0x90, 0xF0,		// 6
			0xF0, 0x10, 0x20, 0x40, 0x40,		// 7
			0xF0, 0x90, 0xF0, 0x90, 0xF0,		// 8
			0xF0, 0x90, 0xF0, 0x10, 0xF0,		// 9
			0xF0, 0x90, 0xF0, 0x90, 0x90,		// A
			0xE0, 0x90, 0xE0, 0x90, 0xE0,		// B
			0xF0, 0x80, 0x80, 0x80, 0xF0,		// C
			0xE0, 0x90, 0x90, 0x90, 0xE0,		// D
			0xF0, 0x80, 0xF0, 0x80, 0xF0,		// E
			0xF0, 0x80, 0xF0, 0x80, 0x80		// F
		]);

		buffer.extend_from_slice(program);

		Memory { 
			buffer
		}
	}

	pub fn get_font_adress(&self, font: u8) -> usize {
		debug_assert!(font < 16);

		usize::from(font) * 5
	}

	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	pub fn read(&self, adress: usize) -> u8 {
		self.buffer[adress]
	}

	pub fn read_bytes(&self, adress: usize, n: usize) -> &[u8] {
		&self.buffer[adress..adress+n]
	}

	pub fn read_opcode(&self, adress: usize) -> u16 {
		let mut op : u16 = 0u16;
        op += u16::from(self.buffer[adress]) << 8;
        op += u16::from(self.buffer[adress + 1]);

		op
	}

	pub fn write(&mut self, adress: usize, byte: u8) {
		self.buffer[adress] = byte;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
	fn test_get_font_adress() {
		let memory = Memory::new(&[]);

		let one_adress = memory.get_font_adress(0x1);
		assert_eq!(memory.read_bytes(one_adress, 5), &[0x20, 0x60, 0x20, 0x20, 0x70]);

		let c_adress = memory.get_font_adress(0xC);
		assert_eq!(memory.read_bytes(c_adress, 5), &[0xF0, 0x80, 0x80, 0x80, 0xF0]);
	}

	#[test]
	fn test_len() {
		let memory = Memory::new(&[0x0, 0x0, 0x0]);

		assert_eq!(memory.len(), 515);
	}

	#[test]
	fn test_read() {
		let memory = Memory::new(&[0xFF, 0xEA, 0x12]);

		assert_eq!(memory.read(512), 0xFF);
		assert_eq!(memory.read(513), 0xEA);
		assert_eq!(memory.read(514), 0x12);
	}

	#[test]
	fn test_read_bytes() {
		let memory = Memory::new(&[0xFF, 0xEA, 0x12, 0x11, 0x54]);

		assert_eq!(memory.read_bytes(513, 3), [0xEA, 0x12, 0x11]);
	}

	#[test]
	fn test_read_opcode() {
		let memory = Memory::new(&[0xFF, 0xEA, 0x12, 0x11, 0x54]);

		assert_eq!(memory.read_opcode(512), 0xFFEA);
		assert_eq!(memory.read_opcode(514), 0x1211);
	}

	#[test]
	fn test_write() {
		let mut memory = Memory::new(&[0x14, 0x13, 0x12, 0x11, 0x10]);

		memory.write(512, 0xFE);
		assert_eq!(memory.read(512), 0xFE);
	}
}