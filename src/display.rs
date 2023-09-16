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
			width,
			height,
			buffer
		}
	}

	pub fn clear(&mut self) {
		self.buffer = vec![0; self.buffer.len()];
	}

	pub fn draw(&mut self, mut x: u8, mut y: u8, sprite: &[u8]) -> bool {
		x %= self.width;
		y %= self.height;

		let mut collide = false;

		let byte_width = usize::from(self.width) / 8;
		let shift = x % 8;
		let first_part = usize::from(x - shift) / 8;
		let second_part = usize::from(x + 8 - shift) / 8;

		for (i, sprite_value) in sprite.iter().enumerate() {
			let y_curr: usize = usize::from(y) + i;
			if y_curr >= usize::from(self.height) {
				break;
			}
			
			let first_before = self.buffer[y_curr * byte_width + first_part];
			let first_after = first_before ^ (sprite_value >> shift);
			self.buffer[y_curr * byte_width + first_part] = first_after;
			
			if second_part < byte_width {
				let mut second_value = *sprite_value;
				for _ in 0..(8 - shift) {
					second_value = (second_value & 0x7F) << 1;
				}
				let second_before = self.buffer[y_curr * byte_width + second_part];
				let second_after = second_before ^ second_value;
				self.buffer[y_curr * byte_width + second_part] = second_after;

				if !collide && check_collide(second_before, second_after) {
					collide = true;
				}
			}
			
			if !collide && check_collide(first_before, first_after) {
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

		(byte >> (8 - 1 - offset)) & 0x01
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

	false
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
    fn test_wrap_draw() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0x12, 0xAE];

        screen.draw(16, 2, &sprite);

        assert_eq!(screen.buffer, vec![0x12, 0x00, 0xAE, 0x00]);
    }

	#[test]
    fn test_clip_draw() {
        let mut screen = Display::new(16, 2);
        let sprite : Vec<u8> = vec![0xFF, 0xFF];

        screen.draw(10, 1, &sprite);

        assert_eq!(screen.buffer, vec![0x00, 0x00, 0x00, 0x3F]);
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