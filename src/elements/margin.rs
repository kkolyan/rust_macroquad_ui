use std::fmt::Debug;
use crate::core::Element;
use crate::elements::group::{GroupFactory, HeightFactory, Layout, Width, WidthFactory};
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
            Node::new().width(offset.left),
            Node::new()
                .add_component(*target.components.get::<Width>().expect("Width required for margin target"))
                .group(Layout::Vertical, vec![
                    Node::new().height(offset.top),
                    target,
                    Node::new().height(offset.bottom),
                ]),
            Node::new().width(offset.right),
        ])
    }
}