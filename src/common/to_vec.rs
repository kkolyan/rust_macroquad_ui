use crate::Node;

pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl <Event, I: Iterator<Item=Node<Event>>> ToVec<Node<Event>> for I {
    fn to_vec(self) -> Vec<Node<Event>> {
        self.collect()
    }
}
