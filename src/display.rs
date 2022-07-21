use crate::events::Direction;
///
/// # Display interface
///
use std::cmp::{max, min};

///
/// Definition of a fonts
///
pub type Font = [(u8, [u8; 5]); 96];

///
/// State of a pixel
///
#[derive(Clone)]
pub enum Pixel {
    On,
    Off,
}

///
/// Basic display interface
///
pub trait Canvas<T: Clone> {
    ///
    /// Width of the display
    ///
    fn width(&self) -> usize;

    ///
    /// Height of the display
    ///
    fn height(&self) -> usize;

    ///
    /// Get display data
    ///
    fn data_size(&self) -> usize;

    ///
    /// Get display data
    ///
    fn data(&self) -> &[u8];

    ///
    /// Data is dirty (has changed since last clear)
    ///
    fn is_dirty(&self) -> bool;

    ///
    /// Clear the data dirty flag
    ///
    fn clear_dirty_flag(&mut self);

    ///
    /// Invert all pixels
    ///
    fn invert(&mut self);

    ///
    /// Invert a row (8 pixels)
    ///
    fn invert_row(&mut self, row: usize);

    ///
    /// Invert part of a row (8 pixels)
    ///
    fn invert_row_slice(&mut self, row: usize, start_col: usize, end_col: usize);

    ///
    /// Fill the entire canvas with a single colour
    ///
    fn fill(&mut self, colour: T);

    ///
    /// Fill an entire row with a single colour
    ///
    fn fill_row(&mut self, row: usize, colour: T);

    /// Fill multiple rows with a single colour
    fn fill_rows(&mut self, start_row: usize, end_row: usize, colour: Pixel);

    ///
    /// Set a pixel
    ///
    fn set_pixel(&mut self, x: usize, y: usize, colour: T);

    ///
    /// Get the state of a pixel
    ///
    fn pixel(&self, x: usize, y: usize) -> Option<T>;

    ///
    /// Copy canvas
    ///
    fn copy_from(&mut self, canvas: &dyn Canvas<T>);

    ///
    /// Print, handles newlines but not scrolling
    ///
    fn print(&mut self, s: &str, row: usize, col: usize, font: &Font, colour: T) {
        let mut row = row;
        let mut col = col;
        for c in s.chars() {
            match c {
                '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += self.print_char(c, row, col, font, colour.clone()) + 1;
                }
            }
        }
    }

    ///
    /// Print character
    ///
    fn print_char(&mut self, t: char, row: usize, col: usize, font: &Font, colour: T) -> usize;

    ///
    /// Vertical scroll the rows in a particular direction
    ///
    fn vscroll_rows(&mut self, row_start: usize, row_end: usize, direction: Direction);
}

///
/// Monochrome display that uses 1bpp for data display.
///
/// Optimally the display width is a multiple of 8.
///
pub struct MonochromeCanvas {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    dirty: bool,
}

impl MonochromeCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        MonochromeCanvas {
            width,
            height,
            buffer: vec![0; (width * height) / 8],
            dirty: true,
        }
    }

    pub fn from_buffer(width: usize, height: usize, buffer: &[u8]) -> Self {
        let buffer_size = (width * height) / 8;
        if buffer.len() != buffer_size {
            panic!("Buffer must be {} bytes long", buffer_size)
        }

        MonochromeCanvas {
            width,
            height,
            buffer: buffer.to_vec(),
            dirty: true,
        }
    }
}

impl Canvas<Pixel> for MonochromeCanvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data_size(&self) -> usize {
        self.buffer.len()
    }

    fn data(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn clear_dirty_flag(&mut self) {
        self.dirty = false;
    }

    fn invert(&mut self) {
        for byte in self.buffer.iter_mut() {
            *byte = !(*byte);
        }
        self.dirty = true;
    }

    ///
    /// Invert a row (8 pixels)
    ///
    fn invert_row(&mut self, row: usize) {
        let start = row * self.width;
        let end = start + self.width;
        for byte in self.buffer[start..end].iter_mut() {
            *byte = !*byte;
        }
    }

    ///
    /// Invert part of a row (8 pixels)
    ///
    fn invert_row_slice(&mut self, row: usize, start_col: usize, end_col: usize) {
        let start = row * self.width + start_col;
        let end = start + (end_col - start_col);
        for byte in self.buffer[start..end].iter_mut() {
            *byte = !*byte;
        }
    }

    ///
    /// Fill the entire display with a Pixel
    ///
    fn fill(&mut self, colour: Pixel) {
        let value = match colour {
            Pixel::On => 0xFFu8,
            Pixel::Off => 0x00u8,
        };

        for byte in self.buffer.iter_mut() {
            *byte = value;
        }

        self.dirty = true;
    }

    ///
    /// Fill the entire canvas with a single colour
    ///
    fn fill_row(&mut self, row: usize, colour: Pixel) {
        let value = match colour {
            Pixel::On => 0xFFu8,
            Pixel::Off => 0x00u8,
        };

        let start = row * self.width;
        let end = start + self.width;
        for byte in self.buffer[start..end].iter_mut() {
            *byte = value;
        }

        self.dirty = true;
    }

    ///
    /// Fill the entire canvas with a single colour
    ///
    fn fill_rows(&mut self, start_row: usize, end_row: usize, colour: Pixel) {
        let value = match colour {
            Pixel::On => 0xFFu8,
            Pixel::Off => 0x00u8,
        };

        let start = start_row * self.width;
        let end = end_row * self.width;
        for byte in self.buffer[start..end].iter_mut() {
            *byte = value;
        }

        self.dirty = true;
    }

    ///
    /// Set a pixel
    ///
    fn set_pixel(&mut self, x: usize, y: usize, colour: Pixel) {
        let width = self.width();
        let height = self.height();
        if (x > width) | (y > height) {
            return;
        }

        let byte_index = (width * (y >> 3)) + x;
        match colour {
            Pixel::On => self.buffer[byte_index] |= 1 << (y & 7),
            Pixel::Off => self.buffer[byte_index] &= !(1 << (y & 7)),
        }

        self.dirty = true;
    }

    ///
    /// Get state of a pixel
    ///
    fn pixel(&self, x: usize, y: usize) -> Option<Pixel> {
        if (x > self.width) | (y > self.height) {
            return None;
        }

        let byte_index = (self.width * (y >> 3)) + x;
        let pixel = self.buffer[byte_index] >> ((y & 7) & 0x01);
        Some(if pixel == 0 { Pixel::Off } else { Pixel::On })
    }

    ///
    /// Copy canvas
    ///
    fn copy_from(&mut self, canvas: &dyn Canvas<Pixel>) {
        self.buffer = canvas.data().to_vec();
    }

    ///
    /// Print single character
    ///
    fn print_char(&mut self, c: char, row: usize, col: usize, font: &Font, colour: Pixel) -> usize {
        let raw = c as usize;
        if raw < 0x20 || raw > 0x7F {
            return 0;
        }
        let char_idx = raw - 0x20;
        let (width, glyph) = font[char_idx];
        for slice in 0..(width as usize) {
            self.buffer[(row * self.width) + col + slice] = match colour {
                Pixel::On => glyph[slice] << 2,
                Pixel::Off => !(glyph[slice] << 2),
            }
        }
        self.dirty = true;
        width as usize
    }

    ///
    /// Vertical scroll the rows in a particular direction
    ///
    fn vscroll_rows(&mut self, row_start: usize, row_end: usize, direction: Direction) {
        let start = min(row_start, row_end) * self.width;
        let end = max(row_start, row_end) * self.width;
        match direction {
            Direction::Up => {
                for row in (start..end).rev() {
                    self.buffer[row + self.width] = self.buffer[row];
                }
                for row in start..(start + self.width) {
                    self.buffer[row] = 0;
                }
            }
            Direction::Down => {
                for row in start..end {
                    self.buffer[row] = self.buffer[row + self.width];
                }
                for row in end..(end + self.width) {
                    self.buffer[row] = 0;
                }
            }
        }
        self.dirty = true;
    }
}
