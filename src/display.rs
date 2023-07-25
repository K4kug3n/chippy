pub struct Display {
	width: u8,
	height: u8,
	buffer: Vec<u8>,
}

impl Display {
	pub fn new(width: u8, height: u8) -> Display {
		debug_assert!(width % 8 == 0);
		let buffer = vec![0; ((usize::from(width) * usize::from(height)) + 7) / 8];

		Display {
			width: width,
			height: height,
			buffer: buffer
		}
	}

	pub fn clear(&mut self) {
		self.buffer = vec![0; self.buffer.len()];
	}

	pub fn draw(&mut self, x: u8, y: u8, sprite: &Vec<u8>) {
		debug_assert!(x < self.width && y < self.height);

		
	}
}