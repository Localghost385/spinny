// --------------------[ Rendering ]-------------------- //

use crate::solids::{Point, Solid};

pub fn render_edges(object: &mut Solid, width: usize, height: usize, scale: f32) -> Vec<Vec<char>> {
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; width]; height];

    pub struct Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    }

    fn draw_line(
        canvas: &mut [Vec<char>],
        width: usize,
        height: usize,
        scale: f32,
        mut line: Line,
    ) {
        line.x1 = ((width / 2) as f32 + line.x1 * scale * 2.0).round();
        line.y1 = ((height / 2) as f32 - line.y1 * scale).round();
        line.x2 = ((width / 2) as f32 + line.x2 * scale * 2.0).round();
        line.y2 = ((height / 2) as f32 - line.y2 * scale).round();

        let dx: f32 = (line.x2 - line.x1).abs();
        let dy: f32 = (line.y2 - line.y1).abs();
        let sx: i8 = if line.x1 < line.x2 { 1 } else { -1 };
        let sy: i8 = if line.y1 < line.y2 { 1 } else { -1 };
        let mut err: f32 = dx - dy;

        loop {
            if (0.0 <= line.x1 && line.x1 < width as f32)
                && (0.0 <= line.y1 && line.y1 < height as f32)
            {
                canvas[line.y1 as usize][line.x1 as usize] = '█';
            }
            if line.x1 == line.x2 && line.y1 == line.y2 {
                break;
            }
            let e2: f32 = err * 2.0;
            if e2 > -dy {
                err -= dy;
                line.x1 += sx as f32;
            }
            if e2 < dx {
                err += dx;
                line.y1 += sy as f32;
            }
        }
    }

    for face in &object.faces {
        let num_points: usize = face.points.len();
        for i in 0..num_points {
            let start_point: &Point = &face.points[i];
            let end_point: &Point = &face.points[(i + 1) % num_points];
            draw_line(
                &mut canvas,
                width,
                height,
                scale,
                Line {
                    x1: start_point.x,
                    y1: start_point.y,
                    x2: end_point.x,
                    y2: end_point.y,
                },
            )
        }
    }
    canvas
}

// ----------------------------------------------------- //

#[cfg(test)]
mod tests {

    use crate::solids::Face;

    use super::*;

    #[test]
    fn test_render_edges() {
        let mut object = Solid::new(vec![
            Face::new(vec![
                Point::new(100.0, 100.0, 100.0),
                Point::new(-100.0, 100.0, 100.0),
                Point::new(-100.0, -100.0, 100.0),
                Point::new(100.0, -100.0, 100.0),
            ]),
            Face::new(vec![
                Point::new(100.0, 100.0, -100.0),
                Point::new(-100.0, 100.0, -100.0),
                Point::new(-100.0, -100.0, -100.0),
                Point::new(100.0, -100.0, -100.0),
            ]),
        ]);
        let expected = [
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', '█', '█', '█', '█', '█', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', '█', ' ', ' ', ' ', '█', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', '█', '█', '█', '█', '█', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        ];

        let result = render_edges(&mut object, 14, 8, 0.01);

        for row in &result {
            for char in 0..row.len() {
                print!("{}", row[char])
            }
            println!()
        }

        assert_eq!(result, expected);
    }
}
