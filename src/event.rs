// --------------------[ Event loop ]-------------------- //
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::thread::sleep;
use std::time::Duration;

use crate::{matrix::rotate_object, renderer::render_edges, solids::Solid};

pub fn rotate_and_display(
    object: &mut Solid,
    theta_x: f32,
    theta_y: f32,
    theta_z: f32,
    delay: f32,
) {
    let delay_duration = Duration::from_millis((delay * 1000.0) as u64);

    loop {
        // Rotate the object
        rotate_object(object, theta_x, theta_y, theta_z);

        // Render the new frame
        let canvas = render_edges(object, 50, 25, 0.06);
        for row in canvas {
            for char in row {
                print!("{}", char)
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
