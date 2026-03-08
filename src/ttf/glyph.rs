use crate::ttf::types::{Reader, int16, uint8, uint16};

#[derive(Debug)]
pub struct GlyphHeader {
    number_of_contours: int16,
    x_min: int16,
    y_min: int16,
    x_max: int16,
    y_max: int16,
}

#[derive(Debug)]
pub struct GlyphPoint {
    x: int16,
    y: int16,
    on_curve: bool,
}

#[derive(Debug)]
pub struct SimpleGlyph {
    header: GlyphHeader,
    end_pts_of_contours: Vec<uint16>,
    instruction_length: uint16,
    instructions: Vec<uint8>,
    points: Vec<GlyphPoint>,
}

// TODO
#[derive(Debug)]
pub struct CompositeGlyph {
    header: GlyphHeader,
}

#[derive(Debug)]
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

        if header.number_of_contours > 0 {
            let end_pts_of_contours: Vec<_> = (0..header.number_of_contours)
                .map(|_| reader.read_uint16())
                .collect();

            let instruction_length = reader.read_uint16();
            let instructions = (0..instruction_length)
                .map(|_| reader.read_uint8())
                .collect();

            let num_points = end_pts_of_contours
                .last()
                .unwrap_or(&0)
                + 1;

            let mut i = 0;
            let mut repeat_count = 0;
            let mut flags: Vec<uint8> = Vec::with_capacity(num_points as usize);
            while i < num_points {
                let flag = reader.read_uint8();
                flags.push(flag);
                i += 1;

                if flag & 0x08 != 0 {
                    repeat_count = reader.read_uint8();
                    for _ in 0..repeat_count {
                        flags.push(flag);
                        i += 1;
                    }
                }
            }

            let mut curr = 0;
            let mut x_coordinates: Vec<int16> = Vec::with_capacity(num_points as usize);
            for &flag in &flags {
                let x_short = flag & 0x02 != 0;
                let x_same = flag & 0x10 != 0;

                if x_short {
                    let x = reader.read_uint8() as int16;
                    let dx = if x_same { x } else { -x };
                    curr += dx;
                } else {
                    if !x_same {
                        let dx = reader.read_int16();
                        curr += dx;
                    }
                }

                x_coordinates.push(curr);
            }

            curr = 0;
            let mut y_coordinates: Vec<int16> = Vec::with_capacity(num_points as usize);
            for &flag in &flags {
                let y_short = flag & 0x04 != 0;
                let y_same = flag & 0x20 != 0;

                if y_short {
                    let y = reader.read_uint8() as int16;
                    let dy = if y_same { y } else { -y };
                    curr += dy;
                } else {
                    if !y_same {
                        let dy = reader.read_int16();
                        curr += dy;
                    }
                }

                y_coordinates.push(curr);
            }

            let points: Vec<GlyphPoint> = x_coordinates.into_iter()
                .zip(y_coordinates.into_iter())
                .zip(flags.into_iter())
                .map(|((x, y), flag)| GlyphPoint {
                    x,
                    y,
                    on_curve: flag & 0x01 != 0,
                })
                .collect();

            Glyph::Simple(SimpleGlyph {
                header,
                end_pts_of_contours,
                instruction_length,
                instructions,
                points,
            })
        } else if header.number_of_contours == 0 {
            // Empty glyph
            Glyph::Simple(SimpleGlyph {
                header,
                end_pts_of_contours: Vec::new(),
                instruction_length: 0,
                instructions: Vec::new(),
                points: Vec::new(),
            })
        } else {
            Glyph::Composite(CompositeGlyph { header })
        }
    }
}
