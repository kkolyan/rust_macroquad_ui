use crate::core::Element;
use crate::elements::node::Node;

#[derive(Debug, Clone, Copy)]
pub struct Name(pub &'static str);

// #[derive(Debug, Clone)]
// pub struct Name(pub String);

pub trait NameFactory<Event> {
   // fn name<S: NodeName>(self, name: S) -> Self;
   fn name(self, name: &'static str) -> Self;
}

impl <Event> NameFactory<Event> for Node<Event> {
    // fn name<S: NodeName>(self, name: S) -> Self {
    //     self.add_component(Name(name.get_node_name()))
    // }
    fn name(self, name: &'static str) -> Self {
        self.add_component(Name(name))
    }
}

// impl <Event> Element<Event> for Name {}
impl <Event> Element<Event> for Name {}


pub trait NodeName {
    fn get_node_name(self) -> String;
}

impl NodeName for String {
    fn get_node_name(self) -> String {
        self
    }
}

impl NodeName for &'static str {
    fn get_node_name(self) -> String {
        self.to_owned()
    }
}

impl NodeName for () {
    fn get_node_name(self) -> String {
        "".to_owned()
    }
}
