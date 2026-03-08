use crate::renderer::pbm::Image;
use crate::ttf::glyph::Glyph;

pub fn render_glyph(glyph: &Glyph) -> Image {
    let header = match glyph {
        Glyph::Simple(simple) => &simple.header,
        Glyph::Composite(composite) => &composite.header,
    };

    let width = (header.x_max - header.x_min) as u32 + 1;
    let height = (header.y_max - header.y_min) as u32 + 1;

    Image::new(width, height)
}
