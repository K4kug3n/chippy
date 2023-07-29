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

	pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
		//debug_assert!(x < self.width && y < self.height);

		let mut collide = false;

		let shift = x % 8;
		for i in 0..sprite.len() {
			let first_part = usize::from(x - shift);
			let first_before = self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + first_part / 8];
			let first_after = first_before ^ (sprite[i] >> shift);
			self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + first_part / 8] = first_after;

			let second_part = usize::from((x + 8 - shift) % self.width);
			let mut second_value = sprite[i];
			for _ in 0..(8 - shift) {
				second_value = (second_value & 0x7F) << 1;
			}
			let second_before = self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + second_part / 8];
			let second_after = second_before ^ second_value;
			self.buffer[((usize::from(y) + i) % usize::from(self.height)) * (usize::from(self.width) / 8) + second_part / 8] = second_after;
			
			if !collide && (check_collide(first_before, first_after) || check_collide(second_before, second_after)) {
				collide = true;
			}

		}

		collide
	}

	pub fn get(&self, x: usize, y: usize) -> u8 
	{
		debug_assert!(x < usize::from(self.width) && y < usize::from(self.height));
		let offset = x % 8;
		let x_idx = (x - offset) / 8;

		let byte = self.buffer[y * usize::from(self.width) / 8 + x_idx];
		let value = (byte >> (8 - 1 - offset)) & 0x01;

		value
	}

	pub fn height(&self) -> u8 {
		self.height
	}

	pub fn width(&self) -> u8 {
		self.width
	}
}

fn check_collide(mut before: u8, mut after: u8) -> bool {
	for _ in 0..8 {
		if (before & 0x1) == 1 && (after & 0x1) == 0 {
			return true
		}

		before >>= 1;
		after >>= 1;
	}

	return false;
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