use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use crate::core::{ComponentSet, Ctx, Element};

#[derive(Debug, Clone)]
pub struct Node<Event> {
    pub name: Rc<RefCell<NodeName>>,
    pub components: ComponentSet<Event>,
}

#[derive(Debug, Clone)]
pub struct NodeName {
    pub own: Option<&'static str>,
    pub parent: Weak<RefCell<NodeName>>,
}

impl<Event> Node<Event> {
    pub fn anon() -> Self {
        Node {
            name: Rc::new(RefCell::new(NodeName { own: None, parent: Weak::new() })),
            components: ComponentSet::new(),
        }
    }
    
    pub fn new(name: &'static str) -> Self {
        Node {
            name: Rc::new(RefCell::new(NodeName { own: Some(name), parent: Weak::new() })),
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
