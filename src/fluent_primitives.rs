use macroquad::color::Color;
use std::fmt::Debug;
use crate::primitives::color_fill::ColorFill;

use crate::primitives::group::Dimension;
use crate::primitives::group::Layout;
use crate::primitives::group::Height;
use crate::primitives::group::Group;
use crate::primitives::group::Width;
use crate::primitives::node::Node;
use crate::primitives::text::Text;
use crate::primitives::text::TextStyle;

pub trait FluentPrimitives<Event> {
    fn color_fill(self, color: Color) -> Self;
    fn width(self, value: f32) -> Self;
    fn width_stretch(self) -> Self;
    fn width_no_stretch(self) -> Self;
    fn height(self, value: f32) -> Self;
    fn height_stretch(self) -> Self;
    fn height_no_stretch(self) -> Self;
    fn layers(self, children: Vec<Node<Event>>) -> Self;
    fn horizontal_group(self, children: Vec<Node<Event>>) -> Self;
    fn vertical_group(self, children: Vec<Node<Event>>) -> Self;
    fn text<S: Into<String>>(self, value: S, style: TextStyle) -> Self;
}

impl<Event: Clone + Debug + 'static> FluentPrimitives<Event> for Node<Event> {
    fn color_fill(self, color: Color) -> Self {
        self.add_component(ColorFill::from(color))
    }

    fn width(self, value: f32) -> Self {
        self.add_component(Width(Dimension::Fixed(value)))
    }

    fn width_stretch(self) -> Self {
        self.add_component(Width(Dimension::Stretch { fixed_part: 0.0 }))
    }

    fn width_no_stretch(self) -> Self {
        self.add_component(Width(Dimension::RemoveStretch))
    }

    fn height(self, value: f32) -> Self {
        self.add_component(Height(Dimension::Fixed(value)))
    }

    fn height_stretch(self) -> Self {
        self.add_component(Height(Dimension::Stretch { fixed_part: 0.0 }))
    }

    fn height_no_stretch(self) -> Self {
        self.add_component(Height(Dimension::RemoveStretch))
    }

    fn layers(self, children: Vec<Node<Event>>) -> Self {
        self.add_component(Group::new(Layout::Layered, children))
    }

    fn horizontal_group(self, children: Vec<Node<Event>>) -> Self {
        self.add_component(Group::new(Layout::Horizontal, children))
    }

    fn vertical_group(self, children: Vec<Node<Event>>) -> Self {
        self.add_component(Group::new(Layout::Vertical, children))
    }

    fn text<S: Into<String>>(self, value: S, style: TextStyle) -> Self {
        self.add_component(Text {
            value: value.into(),
            style,
        })
    }
}
