///
/// Simple tool to parse a 160x24 pixel grid into a font glyph lookup table
///
/// All Glyphs are formatted for easy insertion into a data table
///
use clap::{AppSettings, Clap};
use std::fs::File;
use std::io::Read;

pub mod bitmap;

const WHITE: bitmap::Colour = bitmap::Colour {
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};
const BLACK: bitmap::Colour = bitmap::Colour {
    red: 0x00,
    green: 0x00,
    blue: 0x00,
};

struct Glyph {
    width: usize,
    data: Vec<u8>,
}

impl Glyph {
    fn from_bitmap(
        bitmap: &bitmap::Bitmap,
        x_offset: usize,
        y_offset: usize,
        width: usize,
        height: usize,
    ) -> Self {
        let mut data: Vec<u8> = Vec::new();

        for x in x_offset..(x_offset + width) {
            match bitmap.pixel(x, y_offset) {
                BLACK | WHITE => {
                    data.push(
                        (0..height)
                            .map(|y| match bitmap.pixel(x, y_offset + y) {
                                BLACK => (1 << y) as u8,
                                _ => 0u8,
                            })
                            .sum(),
                    );
                }
                _ => {}
            }
        }

        Glyph {
            width: data.len(),
            data,
        }
    }
}

fn generate_glyphs(bm: bitmap::Bitmap, glyph_width: usize, glyph_height: usize) -> Vec<Glyph> {
    let mut glyphs: Vec<Glyph> = Vec::new();
    for y in 0..(bm.height() / glyph_height) {
        for x in 0..(bm.width() / glyph_width) {
            glyphs.push(Glyph::from_bitmap(
                &bm,
                x * glyph_width,
                y * glyph_height,
                glyph_width,
                glyph_height,
            ));
        }
    }
    glyphs
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Tim Savage <tim@savage.company>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    file_path: String,
    #[clap(short, long, default_value = "5")]
    width: usize,
    #[clap(short, long, default_value = "5")]
    height: usize,
}

fn main() -> Result<(), bitmap::Error> {
    let opts: Opts = Opts::parse();

    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(opts.file_path)?;
    file.read_to_end(&mut buffer)?;

    let glyphs = generate_glyphs(
        bitmap::Bitmap::read_from_buffer(buffer)?,
        opts.width,
        opts.height,
    );

    println!(
        "pub const FONT: [(u8, [u8; {}]); {}] = [",
        opts.width,
        glyphs.len()
    );
    for idx in 0..glyphs.len() {
        let mut slices = glyphs[idx].data.clone();
        for _ in 0..(opts.width - glyphs[idx].width) {
            slices.push(0);
        }

        println!(
            "    ({}, [{}]),  // {}",
            glyphs[idx].width,
            slices
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(", "),
            ((0x20 + idx) as u8) as char
        );
    }
    println!("];");

    Ok(())
}
