use crate::solids::Solid;

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
pub fn rotate_object(object: &mut Solid, theta_x: f32, theta_y: f32, theta_z: f32) {
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

#[cfg(test)]
mod tests {

    use crate::solids::{Face, Point};

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
}
