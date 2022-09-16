use std::fmt::Debug;
use crate::primitives::{height_no_stretch, single, width_no_stretch};
use crate::primitives::node::{Node, node, NodePadding};

#[derive(Copy, Clone, Debug)]
pub enum NoStretchMode {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Debug)]
pub struct NoStretch {
    mode: NoStretchMode,
}

impl<Event: Clone + Debug + 'static> NodePadding<Event> for NoStretch {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        let de_stretch = match self.mode {
            NoStretchMode::Vertical => node()
                .set(height_no_stretch()),
            NoStretchMode::Horizontal => node()
                .set(width_no_stretch()),
            NoStretchMode::Both => node()
                .set(height_no_stretch())
                .set(width_no_stretch()),
        };
        de_stretch.set(single(content))
    }
}

pub fn no_stretch(mode: NoStretchMode) -> NoStretch {
    NoStretch { mode }
}