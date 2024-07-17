use smart_leds::RGB8;

pub struct Lightstrip {
    pub leds: Vec<RGB8>,
    current: i32,
}

impl Lightstrip {
    pub fn new(num_leds: i32) -> Self {
        Self {
            leds: vec![RGB8 { r: 0, g: 0, b: 0 }, num_leds],
            current: 0,
        }
    }

    pub fn set(self: &mut Self, p: RGB8) {
        self.leds[current] = p;
    }

    pub fn next(self: &mut Self) {
        self.current += 1;
    }
}
