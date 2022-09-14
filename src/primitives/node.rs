use std::fmt::Debug;
use crate::core::{ComponentSet, Ctx, Element};
use crate::primitives::node2::{NodeChain, NodeComponent};

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    pub components: ComponentSet<Event>,
}

impl<Event: Clone> Node<Event> {
    pub fn anon() -> Self {
        Node {
            name: None,
            components: ComponentSet::new(),
        }
    }
    pub fn new(name: &'static str) -> Self {
        Node {
            name: Some(name),
            components: ComponentSet::new(),
        }
    }

    pub fn add_component<T: Element<Event> + Clone + Debug + 'static>(mut self, feature: T) -> Self {
        self.components.put(feature);
        self
    }
}

impl<Event: Clone> NodeComponent<Event> for Node<Event> {
    fn do_phase_(&self, ctx: Ctx<Event>, next: NodeChain<Event>) {
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
        next.do_phase(ctx)
    }
}
