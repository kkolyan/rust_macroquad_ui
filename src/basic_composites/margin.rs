use std::fmt::Debug;
use crate::primitives::group::Group;
use crate::primitives::{height, horizontal_group, vertical_group, width};
use crate::primitives::node::{Node, node};

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

pub fn margin<Event: 'static + Clone + Debug, T: Into<MarginOffset>>(t: T, target: Node<Event>) -> Group<Event> {
    let offset = t.into();
    horizontal_group(vec![
            node("frame left")
                .tag(width(offset.left))
                .tag(height(0.0)),
            node("frame central column")
                // .add_component(*target.components.get::<Width>().unwrap_or_else(|| panic!("Width required for margin target {}", target.name.unwrap_or("<node>"))))
                .set(vertical_group(vec![
                    node("frame top")
                        .tag(height(offset.top))
                        .tag(width(0.0)),
                    target,
                    node("frame bottom")
                        .tag(height(offset.bottom))
                        .tag(width(0.0)),
                ])),
            node("frame right")
                .tag(width(offset.right))
                .tag(height(0.0)),
        ])
}
