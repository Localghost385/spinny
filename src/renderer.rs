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

    fn draw_line(canvas: &mut [Vec<char>], width: usize, height: usize, scale: f32, line: Line) {
        // Transform coordinates
        let x1 = ((width as f32 / 2.0) + line.x1 * scale * 2.0).round() as isize;
        let y1 = ((height as f32 / 2.0) - line.y1 * scale).round() as isize;
        let x2 = ((width as f32 / 2.0) + line.x2 * scale * 2.0).round() as isize;
        let y2 = ((height as f32 / 2.0) - line.y2 * scale).round() as isize;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let mut x = x1;
        let mut y = y1;

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;
        let two_dx = 2 * dx;
        let two_dy = 2 * dy;
        let mut e2;

        let mut d = 2 * err - if dx > dy { dy } else { dx };
        let twovdxdy = 2 * (two_dx - two_dy);

        loop {
            // Plot the main pixel
            plot_pixel(canvas, width, height, x, y, d);

            if x == x2 && y == y2 {
                break;
            }

            e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }

            d += twovdxdy;
        }
    }

    fn plot_pixel(
        canvas: &mut [Vec<char>],
        width: usize,
        height: usize,
        x: isize,
        y: isize,
        d: isize,
    ) {
        if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
            let intensity = compute_intensity(d);
            let pixel_char = match intensity {
                0..=8 => '▓',
                _ => '█',
            };
            canvas[y as usize][x as usize] = pixel_char;
        }
    }

    fn compute_intensity(d: isize) -> isize {
        // Compute pixel intensity based on distance
        let max_intensity = 255;
        let distance_factor = (d.abs() as f32 / max_intensity as f32).min(1.0);
        (max_intensity as f32 * (1.0 - distance_factor)) as isize
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
