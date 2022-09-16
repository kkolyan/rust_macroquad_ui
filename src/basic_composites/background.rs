use std::fmt::Debug;
use macroquad::color::Color;
use crate::primitives::{color_fill, single};
use crate::primitives::node::{Node, node, NodePadding};

#[derive(Copy, Clone, Debug)]
pub struct Background {
    pub color: Color,
}

impl <Event: Clone + Debug + 'static> NodePadding<Event> for Background {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        node()
            .name("background")
            .set(color_fill(self.color))
            .set(single(content))
    }
}

pub fn background(color: Color) -> Background {
    Background { color }
}
