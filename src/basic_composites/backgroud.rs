use std::fmt::Debug;
use macroquad::color::Color;
use crate::fluent_primitives::FluentPrimitives;
use crate::primitives::node::{Node, node};

pub struct Background {
    color_fill: Color,
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background { color_fill: color }
    }
}

pub trait FluentBackground {
    fn wrap_background<T: Into<Background>>(self, config: T) -> Self;
}

impl<Event: Clone + Debug + 'static> FluentBackground for Node<Event> {
    fn wrap_background<T: Into<Background>>(self, config: T) -> Self {
        node("background")
            .color_fill(config.into().color_fill)
            .horizontal_group(vec![self])
    }
}