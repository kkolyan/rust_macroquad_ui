use std::fmt::Debug;
use crate::core::{Ctx, Element};
use crate::make_bounded_composite;

make_bounded_composite! {, ComponentSet<Event>, Element<Event>}

#[derive(Debug, Clone)]
pub struct Node<Event> {
    name: Option<&'static str>,
    components: ComponentSet<Event>,
}

impl<Event> Node<Event> {
    pub(crate) fn get<T: 'static + Element<Event>>(&self) -> Option<&T> {
        self.components.get()
    }
}

pub fn node<Event>() -> Node<Event> {
    Node {
        name: None,
        components: ComponentSet::new(),
    }
}

impl<Event: Clone> Node<Event> {

    pub fn set<T: Element<Event> + Clone + Debug + 'static>(mut self, c: T) -> Self {
        self.components.put(c);
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub(crate) fn do_phase(&self, ctx: Ctx<Event>) {
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
    }

    pub(crate) fn get_name(&self) -> Option<&'static str> {
        self.name
    }
}
