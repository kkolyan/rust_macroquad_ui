use std::fmt::Debug;
use crate::fluent_primitives::FluentPrimitives;
use crate::primitives::node::Node;

pub trait FluentMargin<Event> {
    fn wrap_margin<T: Into<MarginOffset>>(self, offset: T) -> Self;
}

pub struct MarginOffset {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    #[allow(dead_code)]
    private: (),
}

impl From<f32> for MarginOffset {
    fn from(margin: f32) -> Self {
        MarginOffset {
            left: margin,
            right: margin,
            bottom: margin,
            top: margin,
            private: (),
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
            private: (),
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
            private: (),
        }
    }
}

impl<Event: Clone + Debug + 'static> FluentMargin<Event> for Node<Event> {
    fn wrap_margin<T: Into<MarginOffset>>(self, offset: T) -> Self {
        let offset = offset.into();
        Node::new("margin").horizontal_group(vec![
            Node::new("frame left").width(offset.left).height(0.0),
            Node::new("frame central column")
                // .add_component(*target.components.get::<Width>().unwrap_or_else(|| panic!("Width required for margin target {}", target.name.unwrap_or("<node>"))))
                .vertical_group(vec![
                    Node::new("frame top").height(offset.top).width(0.0),
                    self,
                    Node::new("frame bottom").height(offset.bottom).width(0.0),
                ]),
            Node::new("frame right").width(offset.right).height(0.0),
        ])
    }
}
