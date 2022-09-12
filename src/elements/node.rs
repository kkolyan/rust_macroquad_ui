use std::fmt::Debug;
use crate::core::{ComponentSet, Ctx, Element};

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    pub components: ComponentSet<Event>,
}

pub trait NodePlugin<Event: Clone>: Element<Event> {
    fn attach_to(self, node: Node<Event>) -> Node<Event>
        where Self: Sized + Clone + Debug + 'static
    {
        node.add_component_raw(self)
    }
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

    pub fn add_component<T: NodePlugin<Event> + Clone + Debug + 'static>(self, feature: T) -> Self {
        feature.attach_to(self)
    }

    pub fn add_component_raw<T: Element<Event> + Clone + Debug + 'static>(mut self, feature: T) -> Self {
        self.components.put(feature);
        self
    }
}

impl<Event: Clone> Element<Event> for Node<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
    }
}
