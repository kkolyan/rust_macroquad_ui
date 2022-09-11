use std::collections::HashSet;
use std::fmt::Debug;

use macroquad::math::Rect;

use node::Node;

use crate::core::Ctx;

pub mod group;
pub mod text;
pub mod mouse;
pub mod icon;
pub mod background;
pub mod border;
pub mod style;
pub mod node;
pub mod name;


impl<Event> Node<Event> {}

#[derive(Debug, Copy, Clone)]
pub enum AlignY {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Copy, Clone)]
pub enum AlignX {
    Left,
    Center,
    Right,
}
