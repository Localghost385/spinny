include!(concat!(env!("OUT_DIR"), "/shape.rs"));

use spinny::{
    event::rotate_and_display,
    solids::{Face, Point, Solid},
};

fn main() {
    let shapes: Solids = Solids::default();

    let selector = "cube";

    let mut shape: Solid;

    match selector {
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

    rotate_and_display(&mut shape, 0.02, 0.025, 0.012, 0.042)
}
