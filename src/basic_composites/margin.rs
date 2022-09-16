use std::fmt::Debug;
use crate::primitives::{height, horizontal_group, vertical_group, width};
use crate::primitives::node::{Node, node, NodePadding};

#[derive(Clone, Debug)]
pub struct Margin {
    offset: MarginOffset,
}

#[derive(Copy, Clone, Debug)]
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

impl<Event: 'static + Clone + Debug> NodePadding<Event> for Margin {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        node()
            .set(
                horizontal_group(vec![
                    node().name("frame left")
                        .set(width(self.offset.left))
                        .set(height(0.0)),
                    node().name("frame central column")
                        .set(vertical_group(vec![
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

pub fn margin<T: Into<MarginOffset>>(t: T) -> Margin {
    let offset = t.into();
    Margin {
        offset
    }
}
