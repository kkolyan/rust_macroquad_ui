use std::fmt::Debug;

use crate::core::{Ctx, Element};
use crate::elements::node::{Node, NodePlugin};

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

impl<Event: Clone> NodePlugin<Event> for DeStretch<Event> {}

pub trait DeStretchFactory<Event> {
    fn de_stretch(self, dimension: DimensionMask, node: Node<Event>) -> Self;
}

impl<Event: 'static + Clone + Debug> DeStretchFactory<Event> for Node<Event> {
    fn de_stretch(self, dimension: DimensionMask, target: Node<Event>) -> Self {
        self.add_component(DeStretch { target, dimension })
    }
}
