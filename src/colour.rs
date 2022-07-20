use rand::{thread_rng, Rng};

const COLOURS: [Colour; 6] = [
    Colour {
        r: 0xFF,
        g: 0x00,
        b: 0x00,
    },
    Colour {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
    },
    Colour {
        r: 0x00,
        g: 0x00,
        b: 0xFF,
    },
    Colour {
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
    },
    Colour {
        r: 0x00,
        g: 0xFF,
        b: 0xFF,
    },
    Colour {
        r: 0xFF,
        g: 0x00,
        b: 0xFF,
    },
];

///
/// Colour definition
///
/// Can represent RGB or Mono colours
///
#[derive(Copy, Clone, Default)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    #[allow(dead_code)]
    pub const BLACK: Colour = Colour { r: 0, g: 0, b: 0 };
    #[allow(dead_code)]
    pub const WHITE: Colour = Colour {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };
    #[allow(dead_code)]
    pub const RED: Colour = Colour { r: 0xFF, g: 0, b: 0 };
    #[allow(dead_code)]
    pub const GREEN: Colour = Colour { r: 0, g: 0xFF, b: 0 };
    #[allow(dead_code)]
    pub const BLUE: Colour = Colour { r: 0, g: 0, b: 0xFF };

    /// Construct a new colour
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Construct a random colour
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen::<u8>(),
            g: rng.gen::<u8>(),
            b: rng.gen::<u8>(),
        }
    }

    pub fn random_indexed() -> Self {
        let mut rng = rand::thread_rng();
        COLOURS[rng.gen_range(0..6)]
    }

    /// Construct a colour from 24bit number
    pub fn from_u24(v: u32) -> Self {
        let r = (v & 0xFF) as u8;
        let g = ((v >> 8) & 0xFF) as u8;
        let b = ((v >> 16) & 0xFF) as u8;
        Self { r, g, b }
    }

    /// "Monochrome" representation of the colour
    pub fn as_1bit(&self) -> u8 {
        if (self.r > 0x7F) | (self.g > 0x7F) | (self.b > 0x7F) {
            0xFF
        } else {
            0x00
        }
    }

    /// Return the components of this colour
    pub fn components(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    ///
    /// Convert colour into a 24bit value
    ///
    pub fn as_u24(&self) -> u32 {
        let c = self.r as u32 | ((self.g as u32) << 8) | ((self.b as u32) << 16);
        c
    }
}
