pub struct Memory {
	buffer: Vec<u8>
}

impl Memory {
	pub fn new(program: &[u8]) -> Memory {
		let mut buffer: Vec<u8> = Vec::from(vec![0; 512]); // Interpreter memory
		buffer.extend_from_slice(program);

		Memory { 
			buffer: buffer
		}
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