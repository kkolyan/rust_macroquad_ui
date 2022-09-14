use std::fmt::Debug;
use crate::core::Ctx;
use crate::fluent_primitives::FluentPrimitives;
use crate::primitives::node2::{Node, NodeChain, NodeComponent};

pub trait FluentMargin<Event> {
    fn margin<T: Into<MarginOffset>>(self, offset: T, target: Node<Event>) -> Self;
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

struct Margin {
    offset: MarginOffset,
}

impl <Event> NodeComponent<Event> for Margin {
    fn do_phase_(&self, ctx: Ctx<Event>, next: NodeChain<Event>) {
        next.do_phase(ctx.clone_with(|it| it.area))
    }
}

impl<Event: Clone + Debug + 'static> FluentMargin<Event> for Node<Event> {
    fn margin<T: Into<MarginOffset>>(self, offset: T, target: Node<Event>) -> Self {
        let offset = offset.into();
        self.horizontal_group(vec![
            Node::new("frame left").width(offset.left).height(0.0),
            Node::new("frame central column")
                // .add_component(*target.components.get::<Width>().unwrap_or_else(|| panic!("Width required for margin target {}", target.name.unwrap_or("<node>"))))
                .vertical_group(vec![
                    Node::new("frame top").height(offset.top).width(0.0),
                    target,
                    Node::new("frame bottom").height(offset.bottom).width(0.0),
                ]),
            Node::new("frame right").width(offset.right).height(0.0),
        ])
    }
}
