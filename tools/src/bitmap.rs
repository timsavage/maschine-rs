///
/// # Cut down bitmap loader
///
/// Only supports Uncompressed 24bpp version 4 bitmaps
///
/// Will ignore colours space info and inverts the pixel buffer to set the origin to the top left.
///
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Formatter;
use std::io::{Cursor, Read, Seek, SeekFrom};

///
/// Potential errors loading images
///
#[derive(Debug)]
pub enum Error {
    IOError(Box<std::io::Error>), // Wrap io::Error
    InvalidSignature,             // Bitmap signature is invalid or unsupported
    UnsupportedFileVersion,       // Invalid or Unsupported bitmap format version
    UnsupportedCompressionMethod, // Invalid or Unsupported compression method
    UnsupportedBitDepth,          // Invalid or Unsupported compression method
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(Box::new(err))
    }
}

#[derive(Debug)]
pub enum BitmapVersion {
    Four,
}

impl BitmapVersion {
    pub fn from_u32(v: u32) -> Result<BitmapVersion, Error> {
        Ok(match v {
            108 => BitmapVersion::Four,
            _ => return Err(Error::UnsupportedFileVersion),
        })
    }
}

#[derive(Debug)]
pub enum CompressionMethod {
    None,
}

impl CompressionMethod {
    pub fn from_u32(v: u32) -> Result<CompressionMethod, Error> {
        Ok(match v {
            0 => CompressionMethod::None,
            _ => return Err(Error::UnsupportedCompressionMethod),
        })
    }
}

#[derive(Debug)]
struct FileHeader {
    file_size: u32,
    // reserved: [u16; 2],
    pixel_data_offset: u32,
}

impl FileHeader {
    fn read(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let file_size = reader.read_u32::<LittleEndian>()?;
        reader.read_u32::<LittleEndian>()?;
        Ok(FileHeader {
            file_size,
            pixel_data_offset: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct DIBHeader {
    version: BitmapVersion,
    width: i32,
    height: i32,
    planes: u16,
    bits_per_pixel: u16,
    compression: CompressionMethod,
    data_size: u32,
    x_ppm: u32,
    y_ppm: u32,
    colour_count: u32,
    important_colour_count: u32,
}

impl DIBHeader {
    fn read(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        Ok(DIBHeader {
            version: BitmapVersion::from_u32(reader.read_u32::<LittleEndian>()?)?,
            width: reader.read_i32::<LittleEndian>()?,
            height: reader.read_i32::<LittleEndian>()?,
            planes: reader.read_u16::<LittleEndian>()?,
            bits_per_pixel: reader.read_u16::<LittleEndian>()?,
            compression: CompressionMethod::from_u32(reader.read_u32::<LittleEndian>()?)?,
            data_size: reader.read_u32::<LittleEndian>()?,
            x_ppm: reader.read_u32::<LittleEndian>()?,
            y_ppm: reader.read_u32::<LittleEndian>()?,
            colour_count: reader.read_u32::<LittleEndian>()?,
            important_colour_count: reader.read_u32::<LittleEndian>()?,
        })
    }

    fn pixel_data_size(&self) -> usize {
        (self.width.abs() * self.height.abs()) as usize
    }

    fn pixel_data_padding(&self) -> i64 {
        (self.width.abs() % 4) as i64
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Bitmap {
    file_header: FileHeader,
    pub dib_header: DIBHeader,
    pixel_buffer: Vec<Colour>,
}

impl Bitmap {
    pub fn read_from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        let mut reader = Cursor::new(buffer);

        check_signature(&mut reader)?;
        let file_header = FileHeader::read(&mut reader)?;
        let dib_header = DIBHeader::read(&mut reader)?;

        // Check on supported bbp are in this file.
        match dib_header.bits_per_pixel {
            4 | 8 | 24 => (),
            _ => return Err(Error::UnsupportedBitDepth),
        }

        let data = read_pixel_data(&mut reader, file_header.pixel_data_offset, &dib_header)?;

        Ok(Bitmap {
            file_header,
            dib_header,
            pixel_buffer: data,
        })
    }

    pub fn width(&self) -> usize {
        self.dib_header.width.abs() as usize
    }

    pub fn height(&self) -> usize {
        self.dib_header.height.abs() as usize
    }

    pub fn pixel(&self, x: usize, y: usize) -> Colour {
        // Inverts the buffer so origin is top-left
        let offset = ((self.height() - y - 1) * self.width()) + x;
        self.pixel_buffer[offset]
    }
}

impl std::fmt::Display for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Width: {}\nHeight: {}\nBitDepth: {}bpp",
            self.width(),
            self.height(),
            self.dib_header.bits_per_pixel
        )
    }
}

const SIGNATURE: u16 = 0x4D_42;

///
/// Check for the Bitmap signature value
///
fn check_signature(reader: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
    let signature = reader.read_u16::<LittleEndian>()?;
    if signature == SIGNATURE {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

///
/// Read in the pixel data (24bit only)
///
fn read_pixel_data(
    reader: &mut Cursor<Vec<u8>>,
    offset: u32,
    dib_header: &DIBHeader,
) -> Result<Vec<Colour>, Error> {
    let mut data: Vec<Colour> = Vec::with_capacity(dib_header.pixel_data_size());

    reader.seek(SeekFrom::Start(offset as u64))?;

    let mut pixels = [0u8; 3];
    for _ in 0..dib_header.height {
        for _ in 0..dib_header.width {
            reader.read(&mut pixels)?;
            data.push(Colour {
                red: pixels[2],
                green: pixels[1],
                blue: pixels[0],
            })
        }
        reader.seek(SeekFrom::Current(dib_header.pixel_data_padding()))?;
    }

    Ok(data)
}
