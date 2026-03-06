use crate::ttf::types::{Reader, int16, uint8, uint16};

pub struct GlyphHeader {
    number_of_contours: int16,
    x_min: int16,
    y_min: int16,
    x_max: int16,
    y_max: int16,
}

pub struct SimpleGlyph {
    header: GlyphHeader,
    end_pts_of_contours: Vec<uint16>,
    instruction_length: uint16,
    instructions: Vec<uint8>,
    flags: Vec<uint8>,
    x_coordinates: Vec<int16>,
    y_coordinates: Vec<int16>,
}

// TODO
pub struct CompositeGlyph {
    header: GlyphHeader,
}

pub enum Glyph {
    Simple(SimpleGlyph),
    Composite(CompositeGlyph),
}

impl Glyph {
    pub fn read_from(reader: &mut Reader) -> Self {
        let header = GlyphHeader {
            number_of_contours: reader.read_int16(),
            x_min: reader.read_int16(),
            y_min: reader.read_int16(),
            x_max: reader.read_int16(),
            y_max: reader.read_int16(),
        };

        if header.number_of_contours >= 0 {
            let end_pts_of_contours = (0..header.number_of_contours)
                .map(|_| reader.read_uint16())
                .collect();

            let instruction_length = reader.read_uint16();
            let instructions = (0..instruction_length)
                .map(|_| reader.read_uint8())
                .collect();

            // TODO read flags and coordinates

            Glyph::Simple(SimpleGlyph {
                header,
                end_pts_of_contours,
                instruction_length,
                instructions,
                flags: vec![],
                x_coordinates: vec![],
                y_coordinates: vec![],
            })
        } else {
            Glyph::Composite(CompositeGlyph { header })
        }
    }
}
