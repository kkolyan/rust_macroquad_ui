use std::fmt::Debug;
use macroquad::color::Color;
use crate::primitives::{color_fill, single_content};
use crate::primitives::node::{Node, node, NodePadding};

#[derive(Copy, Clone, Debug)]
pub struct Background {
    pub color: Color,
}

impl<Event: Clone + Debug + 'static> NodePadding<Event> for Background {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        node()
            .name("background")
            .set(color_fill(self.color))
            .set(single_content(content))
    }
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background { color }
    }
}

impl From<&Color> for Background {
    fn from(color: &Color) -> Self {
        Background { color: *color }
    }
}

pub fn background<T: Into<Background>>(v: T) -> Background {
    v.into()
}
