use std::fmt::Debug;
use crate::basic_composites::background::{Background, background};

use crate::basic_composites::margin::Margin;
use crate::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use crate::Node;
use crate::primitives::{height, horizontal_content, vertical_content, width};
use crate::primitives::node::{node, NodePadding};

pub fn horizontal_node<Event: 'static + Clone + Debug, T: Into<Vec<Node<Event>>>>(items: T) -> Node<Event> {
    node()
        .name("horizontal node")
        .set(horizontal_content(items))
}

pub fn vertical_node<Event: 'static + Clone + Debug, T: Into<Vec<Node<Event>>>>(items: T) -> Node<Event> {
    node()
        .name("vertical node")
        .set(vertical_content(items))
}

pub fn margin_node<Event: Clone + Debug + 'static, T: Into<Margin>>(t: T, target: Node<Event>) -> Node<Event> {
    t.into().expand_padding(target)
}

pub fn no_stretch_node<Event: Clone + Debug + 'static>(mode: NoStretchMode, target: Node<Event>) -> Node<Event> {
    no_stretch(mode).expand_padding(target)
}

pub fn background_node<Event: Clone + Debug + 'static, T: Into<Background>>(t: T, target: Node<Event>) -> Node<Event> {
    background(t).expand_padding(target)
}

pub fn width_node<Event: Clone>(value: f32) -> Node<Event> {
    node()
        .name("width")
        .set(width(value))
        .set(height(0.0))
}

pub fn height_node<Event: Clone>(value: f32) -> Node<Event> {
    node()
        .name("height")
        .set(width(0.0))
        .set(height(value))
}
