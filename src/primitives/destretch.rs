use std::fmt::Debug;

use crate::core::{Ctx, Element};
use crate::primitives::node::Node;

#[derive(Clone, Debug, Copy)]
pub enum DimensionMask {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Debug)]
pub struct DeStretch<Event> {
    pub target: Node<Event>,
    pub dimension: DimensionMask,
}

impl<Event: Clone> Element<Event> for DeStretch<Event> {
    fn do_phase(&self, _ctx: Ctx<Event>) {
        self.target.do_phase(_ctx);
    }
}
