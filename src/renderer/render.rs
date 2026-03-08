use crate::renderer::bazier::{flatten_contour, Point};
use crate::renderer::pbm::Image;
use crate::ttf::glyph::Glyph;

pub fn render_glyph(glyph: &Glyph) -> Image {
    let g = match glyph {
        Glyph::Simple(simple) => simple,
        Glyph::Composite(_) => {
            panic!("Composite glyphs are not supported yet");
        }
    };

    let width = (g.header.x_max - g.header.x_min) as u32 + 1;
    let height = (g.header.y_max - g.header.y_min) as u32 + 1;

    let mut image = Image::new(width, height);

    let points = flatten_contour(&g.points);

    for y in 0..height {
        let ay = g.header.y_max as f64 - y as f64 - 0.5;

        let mut intersections: Vec<f64> = points.windows(2)
            .filter_map(|window| {
                let p0 = window[0];
                let p1 = window[1];

                if (p0.y - ay) * (p1.y - ay) < 0.0 {
                    // 線分がスキャンラインと交差する場合
                    let t = (ay - p0.y) / (p1.y - p0.y);
                    Some(p0.x + t * (p1.x - p0.x))
                } else {
                    None
                }
            })
            .collect();

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in intersections.chunks(2) {
            if let [start, end] = pair {
                let s = start.ceil() as u32;
                let e = end.floor() as u32;

                for x in s..=e {
                    image.set_pixel(x, y, true);
                }
            }
        }
    }

    image
}