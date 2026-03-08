use crate::ttf::glyph::GlyphPoint;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// (1-t)^2 * p0 + 2(1-t)t * p1 + t^2 * p2
fn bazier_2(p0: Point, p1: Point, p2: Point, t: f64) -> Point {
    let u = 1.0 - t;
    Point {
        x: u * u * p0.x + 2.0 * u * t * p1.x + t * t * p2.x,
        y: u * u * p0.y + 2.0 * u * t * p1.y + t * t * p2.y,
    }
}

const NUM_STEPS: i32 = 10;

pub fn flatten_contour(points: &[GlyphPoint]) -> Vec<Point> {
    let mut result = Vec::new();

    if points.is_empty() {
        return result;
    }

    let len = points.len();

    for i in 0..len {
        let curr = &points[i];
        let next = &points[(i + 1) % len];

        if curr.on_curve {
            result.push(Point{
                x: curr.x as f64,
                y: curr.y as f64,
            });

            if next.on_curve {
                // on -> on は直線
                continue
            }
        } else {
            // ベジェ曲線の制御点である場合

            let prev = &points[(i + len - 1) % len];

            // off -> off の場合，中点を視点とする
            let p0 = if prev.on_curve {
                Point {
                    x: prev.x as f64,
                    y: prev.y as f64,
                }
            } else {
                Point {
                    x: (prev.x + curr.x) as f64 / 2.0,
                    y: (prev.y + curr.y) as f64 / 2.0,
                }
            };

            let p1 = Point {
                x: curr.x as f64,
                y: curr.y as f64,
            };

            let p2 = if next.on_curve {
                Point {
                    x: next.x as f64,
                    y: next.y as f64,
                }
            } else {
                Point {
                    x: (curr.x + next.x) as f64 / 2.0,
                    y: (curr.y + next.y) as f64 / 2.0,
                }
            };

            // on-curve のときに点を追加するので、ベジェ曲線の開始点は追加しない
            for step in 1..NUM_STEPS {
                let t = step as f64 / NUM_STEPS as f64;
                result.push(bazier_2(p0, p1, p2, t));
            }
        }
    }

    result
}