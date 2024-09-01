use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::thread::sleep;
use std::time::Duration;

// --------------------[ Classes ]-------------------- //

/// An individual point type
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn apply_matrix(&mut self, matrix: &[Vec<f32>]) {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        self.x = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z;
        self.y = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z;
        self.z = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z;
    }
}

/// A collection of points representing a face
#[derive(Debug, PartialEq, Clone)]
struct Face {
    points: Vec<Point>,
}

impl Face {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

/// A collection of faces representing a solid
#[derive(Debug, PartialEq, Clone)]
struct Solid {
    faces: Vec<Face>,
}

impl Solid {
    fn new(faces: Vec<Face>) -> Self {
        Self { faces }
    }
}

struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

// --------------------------------------------------- //

// --------------------[ Matrix Transformations ]-------------------- //

/// Calculate matrix for rotation on the x axis
fn rotation_matrix_x(theta: f32) -> Vec<Vec<f32>> {
    vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, theta.cos(), -theta.sin()],
        vec![0.0, theta.sin(), theta.cos()],
    ]
}

/// Calculate matrix for rotation on the y axis
fn rotation_matrix_y(theta: f32) -> Vec<Vec<f32>> {
    vec![
        vec![theta.cos(), 0.0, theta.sin()],
        vec![0.0, 1.0, 0.0],
        vec![-theta.sin(), 0.0, theta.cos()],
    ]
}

/// Calculate matrix for rotation on the z axis
fn rotation_matrix_z(theta: f32) -> Vec<Vec<f32>> {
    vec![
        vec![theta.cos(), -theta.sin(), 0.0],
        vec![theta.sin(), theta.cos(), 0.0],
        vec![0.0, 0.0, 1.0],
    ]
}

/// Calculate the matrices and rotate the object
fn rotate_object(object: &mut Solid, theta_x: f32, theta_y: f32, theta_z: f32) {
    let rx = rotation_matrix_x(theta_x);
    let ry = rotation_matrix_y(theta_y);
    let rz = rotation_matrix_z(theta_z);

    for face in &mut object.faces {
        for point in &mut face.points {
            point.apply_matrix(&rx);
            point.apply_matrix(&ry);
            point.apply_matrix(&rz);
        }
    }
}

// ------------------------------------------------------------------ //

// --------------------[ Rendering ]-------------------- //

fn render_edges(object: &mut Solid, width: usize, height: usize, scale: f32) -> Vec<Vec<char>> {
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; width]; height];

    fn draw_line(
        canvas: &mut Vec<Vec<char>>,
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

// --------------------[ Event loop ]-------------------- //

fn rotate_and_display(object: &mut Solid, theta_x: f32, theta_y: f32, theta_z: f32, delay: f32) {
    let delay_duration = Duration::from_millis((delay * 1000.0) as u64);

    loop {
        // Rotate the object
        rotate_object(object, theta_x, theta_y, theta_z);

        // Render the new frame
        let canvas = render_edges(object, 50, 25, 0.06);
        for row in &canvas {
            for char in 0..row.len() {
                print!("{}", row[char])
            }
            println!()
        }

        // Move the cursor up by 25 lines to start drawing at the top again
        execute!(std::io::stdout(), cursor::MoveUp(25)).unwrap();

        // Clear the previous cube drawing
        execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();

        // Wait for the specified delay duration
        sleep(delay_duration);
    }
}

// ------------------------------------------------------ //

fn main() {
    let mut cube = Solid::new(vec![
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
        Face::new(vec![
            Point::new(100.0, 100.0, 100.0),
            Point::new(100.0, -100.0, 100.0),
            Point::new(100.0, -100.0, -100.0),
            Point::new(100.0, 100.0, -100.0),
        ]),
        Face::new(vec![
            Point::new(-100.0, 100.0, 100.0),
            Point::new(-100.0, -100.0, 100.0),
            Point::new(-100.0, -100.0, -100.0),
            Point::new(-100.0, 100.0, -100.0),
        ]),
        Face::new(vec![
            Point::new(100.0, 100.0, 100.0),
            Point::new(-100.0, 100.0, 100.0),
            Point::new(-100.0, 100.0, -100.0),
            Point::new(100.0, 100.0, -100.0),
        ]),
        Face::new(vec![
            Point::new(100.0, -100.0, 100.0),
            Point::new(-100.0, -100.0, 100.0),
            Point::new(-100.0, -100.0, -100.0),
            Point::new(100.0, -100.0, -100.0),
        ]),
    ]);
    rotate_and_display(&mut cube, 0.02, 0.025, 0.012, 0.042)
}

#[cfg(test)]
mod tests {

    use super::*;

    const EPSILON: f32 = 1e-6; // Allowable error margin

    // Helper function to compare two matrices
    fn assert_matrix_approx_eq(a: &[Vec<f32>], b: &[Vec<f32>]) {
        for (row_a, row_b) in a.iter().zip(b.iter()) {
            for (&val_a, &val_b) in row_a.iter().zip(row_b.iter()) {
                assert!(
                    (val_a - val_b).abs() < EPSILON,
                    "Matrix mismatch: {} != {}",
                    val_a,
                    val_b
                );
            }
        }
    }

    #[test]
    fn test_apply_matrix() {
        let matrix: Vec<Vec<f32>> = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.7071068, -0.7071068],
            vec![0.0, 0.7071068, 0.7071068],
        ];
        let mut result: Point = Point::new(100.0, 100.0, 100.0);
        result.apply_matrix(&matrix);
        let expected = Point::new(100.0, 0.0, 141.42137);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_rotation_matrix_x() {
        let theta = std::f32::consts::PI / 4.0; // 45 degrees
        let expected = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.7071068, -0.7071068],
            vec![0.0, 0.7071068, 0.7071068],
        ];
        let result = rotation_matrix_x(theta);
        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn test_rotation_matrix_y() {
        let theta = std::f32::consts::PI / 4.0; // 45 degrees
        let expected = vec![
            vec![0.7071068, 0.0, 0.7071068],
            vec![0.0, 1.0, 0.0],
            vec![-0.7071068, 0.0, 0.7071068],
        ];
        let result = rotation_matrix_y(theta);
        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn test_rotation_matrix_z() {
        let theta = std::f32::consts::PI / 4.0; // 45 degrees
        let expected = vec![
            vec![0.7071068, -0.7071068, 0.0],
            vec![0.7071068, 0.7071068, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let result = rotation_matrix_z(theta);
        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn test_rotate_object() {
        let theta = std::f32::consts::PI / 4.0; // 45 degrees
        let mut result = Solid::new(vec![
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
        rotate_object(&mut result, theta, theta, theta);
        let expected = Solid::new(vec![
            Face::new(vec![
                Point::new(120.71068, 120.71068, 29.289322),
                Point::new(20.710678, 20.710678, 170.71068),
                Point::new(50.0, -150.0, 70.71068),
                Point::new(150.0, -50.0, -70.71068),
            ]),
            Face::new(vec![
                Point::new(-50.0, 150.0, -70.71068),
                Point::new(-150.0, 50.0, 70.71068),
                Point::new(-120.71068, -120.71068, -29.289322),
                Point::new(-20.710678, -20.710678, -170.71068),
            ]),
        ]);

        assert_eq!(result, expected)
    }

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
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '█', '█', '█', '█', '█', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '█', ' ', ' ', ' ', '█', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '█', '█', '█', '█', '█', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            [
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
        ];

        let result = render_edges(&mut object, 20, 10, 0.01);

        for row in &result {
            for char in 0..row.len() {
                print!("{}", row[char])
            }
            println!()
        }

        assert_eq!(result, expected);
    }
}
