const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;

pub struct Display {
    screen: [[u8; WIDTH as usize]; HEIGHT as usize]
}

// Related Functions
impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH as usize]; HEIGHT as usize]
        }
    }
}

// Methods
impl Display {
    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {

        let mut flipped: bool = false;

        let y_cord = y as usize;
        
        for k in 0..8 {
            let x_cord: usize = (x as usize) + k;
            let bit = (byte >> (7 - k)) & 0b0000_0001;
            let prev = self.screen[y_cord][x_cord];
            match bit {
                0 => self.screen[y_cord][x_cord] = 1,
                1 => self.screen[y_cord][x_cord] = 0,
                _ => unreachable!()
            }
            if prev != self.screen[y_cord][x_cord] {
                flipped = true;
            }
        }
        flipped
    }

    pub fn clear(&mut self) {
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                self.screen[x][y] = 0;
            }
        }
    }
}