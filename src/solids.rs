// --------------------[ Classes ]-------------------- //

/// An individual point type
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn apply_matrix(&mut self, matrix: &[Vec<f32>]) {
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
pub struct Face {
    pub points: Vec<Point>,
}

impl Face {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

/// A collection of faces representing a solid
#[derive(Debug, PartialEq, Clone)]
pub struct Solid {
    pub faces: Vec<Face>,
}

impl Solid {
    pub fn new(faces: Vec<Face>) -> Self {
        Self { faces }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
}

// --------------------------------------------------- //
