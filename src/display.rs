use bit_vec::BitVec;

pub struct Display {
	width: u8,
	height: u8,
	buffer: BitVec,
}

impl Display {
	pub fn new(width: u8, height: u8) -> Display {
		Display {
			width: width,
			height: height,
			buffer: BitVec::from_elem((width * height).into(), true)
		}
	}

	pub fn clear(&mut self) {
		self.buffer.set_all();

	}

	pub fn flip(&mut self, x: u8, y: u8) {
		debug_assert!(x < self.width && y < self.height);

		let index : usize = (y * self.width + x).into();
		let old_value = self.buffer.get(index).unwrap();

		self.buffer.set(index, !old_value);
	}
}