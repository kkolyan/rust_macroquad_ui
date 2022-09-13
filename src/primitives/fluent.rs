use macroquad::color::Color;
use std::fmt::Debug;
use crate::primitives::background::Background;

use crate::primitives::group::Size;
use crate::primitives::group::Layout;
use crate::primitives::group::Height;
use crate::primitives::group::Group;
use crate::primitives::group::Width;
use crate::primitives::margin::MarginOffset;
use crate::primitives::node::Node;
use crate::primitives::text::Text;
use crate::primitives::text::TextStyle;
use crate::primitives::text::ToText;

pub trait PrimitivesFluentFactory<Event> {
    fn background_from_color(self, color: Color) -> Self;
    fn width(self, value: f32) -> Self;
    fn width_stretch(self) -> Self;
    fn width_no_stretch(self) -> Self;
    fn height(self, value: f32) -> Self;
    fn height_stretch(self) -> Self;
    fn height_no_stretch(self) -> Self;
    fn group(self, layout: Layout, children: Vec<Node<Event>>) -> Self;
    fn margin(self, offset: MarginOffset, target: Node<Event>) -> Self;
    fn text<S: ToText>(self, value: S, style: TextStyle) -> Self;
}

impl<Event: Clone + Debug + 'static> PrimitivesFluentFactory<Event> for Node<Event> {
    fn background_from_color(self, color: Color) -> Self {
        self.add_component(Background::from(color))
    }

    fn width(self, value: f32) -> Self {
        self.add_component(Width(Size::Fixed(value)))
    }

    fn width_stretch(self) -> Self {
        self.add_component(Width(Size::Stretch { fixed_part: 0.0 }))
    }

    fn width_no_stretch(self) -> Self {
        self.add_component(Width(Size::RemoveStretch))
    }

    fn height(self, value: f32) -> Self {
        self.add_component(Height(Size::Fixed(value)))
    }

    fn height_stretch(self) -> Self {
        self.add_component(Height(Size::Stretch { fixed_part: 0.0 }))
    }

    fn height_no_stretch(self) -> Self {
        self.add_component(Height(Size::RemoveStretch))
    }

    fn group(self, layout: Layout, children: Vec<Node<Event>>) -> Self {
        self.add_component(Group::new(layout, children))
    }

    fn margin(self, offset: MarginOffset, target: Node<Event>) -> Self {
        self.group(Layout::Horizontal, vec![
            Node::new("frame left").width(offset.left).height(0.0),
            Node::new("frame central column")
                // .add_component(*target.components.get::<Width>().unwrap_or_else(|| panic!("Width required for margin target {}", target.name.unwrap_or("<node>"))))
                .group(Layout::Vertical, vec![
                    Node::new("frame top").height(offset.top).width(0.0),
                    target,
                    Node::new("frame bottom").height(offset.bottom).width(0.0),
                ]),
            Node::new("frame right").width(offset.right).height(0.0),
        ])
    }

    fn text<S: ToText>(self, value: S, style: TextStyle) -> Self {
        let text = Text {
            value: value.to_text(),
            style,
        };
        let size = text.measure_self();
        self.add_component(text)
            .width(size.width)
            .height(style.font_size)// because size.y varies depends on the presence of letters like "p"
    }
}
