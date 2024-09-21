include!(concat!(env!("OUT_DIR"), "/shape.rs"));

use clap::Parser;
use spinny::{cli::CliOptions, event::rotate_and_display, solids::Solid};

fn main() {
    let shapes: Solids = Solids::default();

    let cli_options = CliOptions::parse();

    let mut shape: Solid;

    match cli_options.shape.as_str() {
        "cube" => {
            shape = shapes.cube;
        }
        "tetrahedron" => {
            shape = shapes.tetrahedron;
        }
        _ => {
            shape = shapes.cube;
        }
    };

    rotate_and_display(
        &mut shape,
        cli_options.x as f32 / 100.0,
        cli_options.y as f32 / 100.0,
        cli_options.z as f32 / 100.0,
        0.042,
    )

    // print!("{:?}",cli_options);
}
