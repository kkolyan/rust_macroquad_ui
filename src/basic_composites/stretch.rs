use crate::primitives::{height, height_stretch, width, width_stretch};
use crate::primitives::node::{Node, node};


pub fn stretch_horizontal<Event: Clone>() -> Node<Event> {
    node().set(width_stretch()).set(height(0.0))
}

pub fn stretch_vertical<Event: Clone>() -> Node<Event> {
    node().set(height_stretch()).set(width(0.0))
}
