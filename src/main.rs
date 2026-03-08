use std::{env, fs};
use crate::renderer::loader::Font;

pub mod renderer;
pub mod ttf;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <font-file>", args[0]);
        return;
    }

    let buf = fs::read(&args[1]).expect("Failed to read font file");

    let mut font = Font::from_data(&*buf);
    font.read_glyf();

    font.render_glyph(0x41); // Render 'A'
}
