use std::fmt::Debug;
use crate::core::{ComponentSet, Ctx, Element};

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    pub components: ComponentSet<Event>,
}

impl<Event> Node<Event> {
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

impl <Event: Clone> Element<Event> for Node<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
    }
}
