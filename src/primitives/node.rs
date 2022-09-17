use std::fmt::Debug;
use std::mem;
use crate::core::{Ctx, Element};
use crate::{make_bounded_any_box, make_bounded_composite};

make_bounded_composite! {, ComponentSet<Event>, Element<Event>}
make_bounded_any_box!{,NodePaddingBox<Event>, NodePadding<Event>}

#[derive(Debug, Clone)]
pub struct Node<Event> {
    name: Option<&'static str>,
    components: ComponentSet<Event>,
    paddings: Vec<NodePaddingBox<Event>>,
}

pub trait NodePadding<Event> {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event>;
}

impl<Event> Node<Event> {
    pub(crate) fn unique<T: 'static + Element<Event>>(&self) -> Option<&T> {
        let option = self.components.get().map(|it| it.collect::<Vec<&T>>());
        if let Some(candidates) = option {
            if candidates.len() > 1 {
                panic!("failed to resolve single component");
            }
            return candidates.get(0).copied();
        }
        None
    }
}

pub fn node<Event>() -> Node<Event> {
    Node {
        name: None,
        components: ComponentSet::new(),
        paddings: vec![],
    }
}

impl<Event: Clone> Node<Event> {

    pub fn set<T: Element<Event> + Clone + Debug + 'static>(mut self, component: T) -> Self {
        let padded = component.expand_padding();
        self.components.insert(padded);
        self
    }

    pub fn pad<T: NodePadding<Event> + Clone + Debug + 'static>(mut self, padding: T) -> Self {
        self.paddings.push(NodePaddingBox::new(padding));
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub(crate) fn expand_padding(self) -> Self {
        let mut node = self;
        let paddings = mem::take(&mut node.paddings);
        for padding in paddings.iter().rev() {
            node = padding.as_ref().expand_padding(node);
        }
        node
    }

    pub(crate) fn do_phase(&self, ctx: Ctx<Event>) {
        let name = self.name.unwrap_or("noname");
        let area = format!("{:?}", ctx.area);
        let x = 0;
        for feature in self.components.iter() {
            feature.do_phase(ctx.clone());
        }
    }

    pub(crate) fn get_name(&self) -> Option<&'static str> {
        self.name
    }
}
