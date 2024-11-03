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

		let shift = x % 8;
		for i in 0..sprite.len() {
			let first_part = usize::from(x - shift);
			self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + first_part / 8] ^= sprite[i] >> shift;

			let second_part = usize::from((x + 8 - shift) % self.width);
			let mut second_value = sprite[i];
			for _ in 0..(8 - shift) {
				second_value = (second_value & 0x7F) << 1;
			}
			self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + second_part / 8] ^= second_value;
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aligned_draw() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0xFF, 0xAE];

        screen.draw(8, 0, &sprite);

        assert_eq!(screen.buffer, vec![0x00, 0xFF, 0x00, 0xAE]);
    }

	#[test]
    fn test_non_aligned_draw() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0x12, 0xAE];

        screen.draw(5, 0, &sprite);

        assert_eq!(screen.buffer, vec![0x00, 0x90, 0x05, 0x70]);
    }

	#[test]
    fn test_overflow_draw() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0xFF, 0xAE];

        screen.draw(8, 1, &sprite);

        assert_eq!(screen.buffer, vec![0x00, 0xAE, 0x00, 0xFF]);
    }

	#[test]
    fn test_clear() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0x12, 0xAE];

        screen.draw(5, 0, &sprite);
		screen.clear();

        assert_eq!(screen.buffer, vec![0x00, 0x00, 0x00, 0x00]);
    }
}