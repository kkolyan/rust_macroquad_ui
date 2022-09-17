use std::fmt::Debug;
use crate::primitives::{height, horizontal_content, vertical_content, width};
use crate::primitives::node::{Node, node, NodePadding};

#[derive(Clone, Debug)]
pub struct Margin {
    offset: MarginOffset,
}

#[derive(Copy, Clone, Debug)]
struct MarginOffset {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl From<f32> for Margin {
    fn from(margin: f32) -> Self {
        (margin, margin).into()
    }
}

impl From<(f32, f32)> for Margin {
    fn from((h, v): (f32, f32)) -> Self {
        (v, h, v, h).into()
    }
}

impl From<(f32, f32, f32, f32)> for Margin {
    //noinspection SpellCheckingInspection
    fn from(trbl: (f32, f32, f32, f32)) -> Self {
        let (top, right, bottom, left) = trbl;
        Margin {
            offset: MarginOffset {
                left,
                right,
                bottom,
                top,
            }
        }
    }
}

impl<Event: 'static + Clone + Debug> NodePadding<Event> for Margin {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        node()
            .name("margin")
            .set(
                horizontal_content([
                    node().name("frame left")
                        .set(width(self.offset.left))
                        .set(height(0.0)),
                    node().name("frame central column")
                        .set(vertical_content([
                            node().name("frame top")
                                .set(height(self.offset.top))
                                .set(width(0.0)),
                            content,
                            node().name("frame bottom")
                                .set(height(self.offset.bottom))
                                .set(width(0.0)),
                        ])),
                    node().name("frame right")
                        .set(width(self.offset.right))
                        .set(height(0.0)),
                ]))
    }
}

pub fn margin<T: Into<Margin>>(t: T) -> Margin {
    t.into()
}
