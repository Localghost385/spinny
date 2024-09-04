
include!(concat!(env!("OUT_DIR"), "/shape.rs"));

use spinny::{
    event::rotate_and_display,
    solids::{Face, Point, Solid},
};

fn main() {
    let mut shapes: Solids = Solids::new();
    rotate_and_display(&mut shapes.cube, 0.02, 0.025, 0.012, 0.042)
}

