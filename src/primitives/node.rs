use std::fmt::Debug;
use crate::core::{ComponentSet, Ctx, Element};
use crate::make_bounded_composite;

make_bounded_composite! {pub, TagSet, TagMarker}

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Option<&'static str>,
    components: ComponentSet<Event>,
    tags: TagSet,
}

pub trait TagMarker {}

impl<Event> Node<Event> {
    pub(crate) fn get_comp<T: 'static + Element<Event>>(&self) -> Option<&T> {
        self.components.get()
    }
    pub(crate) fn get_tag<T: TagMarker + 'static>(&self) -> Option<&T> {
        self.tags.get()
    }
}

pub fn node<Event>(name: &'static str) -> Node<Event> {
    Node {
        name: Some(name),
        components: ComponentSet::new(),
        tags: TagSet::new(),
    }
}

impl<Event: Clone> Node<Event> {
    pub fn anon() -> Self {
        Node {
            name: None,
            components: ComponentSet::new(),
            tags: TagSet::new(),
        }
    }

    pub fn set<T: Element<Event> + Clone + Debug + 'static>(mut self, c: T) -> Self {
        self.components.put(c);
        self
    }

    pub fn tag<T: TagMarker + Clone + Debug + 'static>(mut self, c: T) -> Self {
        self.tags.put(c);
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
