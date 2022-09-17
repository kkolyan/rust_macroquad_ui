use macroquad::color::Color;
use crate::primitives::color_fill::ColorFill;
use crate::primitives::group::{Dimension, Group, Height, Layout, Width};
use crate::primitives::node::Node;
use crate::primitives::text::{Text, TextStyle};

pub mod group;
pub mod text;
pub mod mouse;
pub mod color_fill;
pub mod border;
pub mod conditional;
pub mod node;
pub mod texture;


pub fn color_fill(color: Color) -> ColorFill {
    ColorFill::from(color)
}

pub fn width(value: f32) -> Width {
    Width(Dimension::Fixed(value))
}

pub fn width_stretch() -> Width {
    Width(Dimension::Stretch { fixed_part: 0.0 })
}

pub fn width_no_stretch() -> Width {
    Width(Dimension::RemoveStretch)
}

pub fn height(value: f32) -> Height {
    Height(Dimension::Fixed(value))
}

pub fn height_stretch() -> Height {
    Height(Dimension::Stretch { fixed_part: 0.0 })
}

pub fn height_no_stretch() -> Height {
    Height(Dimension::RemoveStretch)
}

pub fn layers<Event>(children: Vec<Node<Event>>) -> Group<Event> {
    Group::new(Layout::Layered, children)
}

pub fn horizontal_group<Event>(children: Vec<Node<Event>>) -> Group<Event> {
    Group::new(Layout::Horizontal, children)
}

pub fn single<Event>(child: Node<Event>) -> Group<Event> {
    Group::new(Layout::Horizontal, vec![child])
}

pub fn vertical_group<Event>(children: Vec<Node<Event>>) -> Group<Event> {
    Group::new(Layout::Vertical, children)
}

pub fn text<S: Into<String>>(value: S, style: TextStyle) -> Text {
    Text { value: value.into(), style }
}