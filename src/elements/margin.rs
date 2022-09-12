use std::fmt::Debug;

use crate::elements::group::GroupFactory;
use crate::elements::group::HeightFactory;
use crate::elements::group::Layout;
use crate::elements::group::WidthFactory;
use crate::elements::node::Node;

pub struct MarginOffset {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl From<f32> for MarginOffset {
    fn from(margin: f32) -> Self {
        MarginOffset {
            left: margin,
            right: margin,
            bottom: margin,
            top: margin,
        }
    }
}

impl From<(f32, f32)> for MarginOffset {
    fn from(hv: (f32, f32)) -> Self {
        let (h, v) = hv;
        MarginOffset {
            left: h,
            right: h,
            bottom: v,
            top: v,
        }
    }
}

impl From<(f32, f32, f32, f32)> for MarginOffset {
    //noinspection SpellCheckingInspection
    fn from(trbl: (f32, f32, f32, f32)) -> Self {
        let (top, right, bottom, left) = trbl;
        MarginOffset {
            left,
            right,
            bottom,
            top,
        }
    }
}

pub trait MarginFactory<Event> {
    fn margin(self, offset: MarginOffset, target: Node<Event>) -> Self;
}

impl<Event: 'static +  Clone + Debug> MarginFactory<Event> for Node<Event> {
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
}