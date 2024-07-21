pub struct Lightstrip {
    pub leds: Vec<(u8, u8, u8)>,
    current: i32,
}

impl Lightstrip {
    pub fn new(num_leds: i32) -> Self {
        Self {
            leds: vec![(0, 0, 0), num_leds],
            current: 0,
        }
    }

    pub fn set(self: &mut Self, p: (u8, u8, u8)) {
        self.leds[current] = p;
    }

    pub fn next(self: &mut Self) {
        self.current += 1;
    }
}
