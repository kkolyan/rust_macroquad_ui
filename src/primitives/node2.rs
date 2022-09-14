use std::fmt::Debug;
use std::iter::{Enumerate, Map};
use std::slice::Iter;
use crate::core::{ComponentSet, Ctx, Element};
use crate::make_bounded_composite;

make_bounded_composite! {pub, NodeComponentSet<Event>, NodeComponent<Event>}

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    pub components: NodeComponentSet<Event>,
}

impl<Event: Clone> Node<Event> {
    pub fn anon() -> Self {
        Node {
            name: None,
            components: NodeComponentSet::new(),
        }
    }
    pub fn new(name: &'static str) -> Self {
        Node {
            name: Some(name),
            components: NodeComponentSet::new(),
        }
    }

    pub fn add_component<T: NodeComponent<Event> + Clone + Debug + 'static>(mut self, feature: T) -> Self {
        self.components.put(feature);
        self
    }
}

pub trait NodeComponent<Event> {
    fn do_phase_(&self, ctx: Ctx<Event>, next: NodeChain<Event>) {
        next.do_phase(ctx)
    }
}

pub struct NodeChain<'a, Event> {
    components: &'a NodeComponentSet<Event>,
    index: usize,
}

impl <'a, Event> NodeChain<'a, Event> {
    pub fn do_phase(self, ctx: Ctx<Event>) {
        if let Some(comp) = self.components.get_i(self.index) {
            comp.do_phase_(ctx, NodeChain {index: self.index + 1, components: self.components})
        }
    }
}

impl<Event: Clone> Element<Event> for Node<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        NodeChain {components: &self.components, index: 0}.do_phase(ctx)
    }
}
