use std::collections::HashSet;
use std::fmt::Debug;
use crate::basic_composites::node_factories::{horizontal_node, vertical_node};
use crate::basic_composites::stretch::StretchSide::{StretchBottom, StretchLeft, StretchRight, StretchTop};
use crate::primitives::{height, height_stretch, width, width_stretch};
use crate::primitives::node::{Node, node, NodePadding};


pub fn stretch_horizontal<Event: Clone>() -> Node<Event> {
    node()
        .name("stretch_horizontal")
        .set(width_stretch())
        .set(height(0.0))
}

pub fn stretch_vertical<Event: Clone>() -> Node<Event> {
    node()
        .name("stretch_vertical")
        .set(height_stretch())
        .set(width(0.0))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StretchSide {
    StretchTop,
    StretchRight,
    StretchBottom,
    StretchLeft,
}

#[derive(Clone, Debug)]
pub struct StretchAround {
    sides: HashSet<StretchSide>,
}

impl<Event: Clone + Debug + 'static> NodePadding<Event> for StretchAround {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        let mut n = content;
        if self.sides.contains(&StretchLeft) || self.sides.contains(&StretchRight) {
            let mut items = vec![];
            if self.sides.contains(&StretchLeft) {
                items.push(stretch_horizontal());
            }
            items.push(n);
            if self.sides.contains(&StretchRight) {
                items.push(stretch_horizontal());
            }
            n = horizontal_node(items)
        }
        if self.sides.contains(&StretchTop) || self.sides.contains(&StretchBottom) {
            let mut items = vec![];
            if self.sides.contains(&StretchTop) {
                items.push(stretch_vertical());
            }
            items.push(n);
            if self.sides.contains(&StretchBottom) {
                items.push(stretch_vertical());
            }
            n = vertical_node(items)
        }
        n
    }
}

pub fn stretch_around<T: Into<HashSet<StretchSide>>>(sides: T) -> StretchAround {
    StretchAround { sides: sides.into() }
}
