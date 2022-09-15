use std::fmt::Debug;
use crate::core::{ComponentSet, Ctx, Element};

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    components: ComponentSet<Event>,
}

impl<Event> Node<Event> {
    pub(crate) fn get<T: 'static + Element<Event>>(&self) -> Option<&T> {
        self.components.get()
    }
}

pub fn node<Event>(name: &'static str) -> Node<Event> {
    Node {
        name: Some(name),
        components: ComponentSet::new(),
    }
}

impl<Event: Clone> Node<Event> {
    pub fn anon() -> Self {
        Node {
            name: None,
            components: ComponentSet::new(),
        }
    }

    pub fn set<T: Element<Event> + Clone + Debug + 'static>(mut self, c: T) -> Self {
        self.components.put(c);
        self
    }
}

impl<Event: Clone> Node<Event> {
    pub(crate) fn do_phase(&self, ctx: Ctx<Event>) {
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
    }
}
