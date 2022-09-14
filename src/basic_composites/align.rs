use std::fmt::Debug;
use crate::fluent_primitives::FluentPrimitives;
use crate::primitives::node::{Node, node};

pub trait FluentAlign<Event> {
    fn wrap_align(self, x: AlignX, y: AlignY) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub enum AlignX {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum AlignY {
    Top,
    Center,
    Bottom,
}

impl<Event: Clone + Debug + 'static> FluentAlign<Event> for Node<Event> {
    fn wrap_align(self, x: AlignX, y: AlignY) -> Self {
        let row = {
            let stretch_x = node("stretch x").width_stretch().height(0.0);
            match x {
                AlignX::Left => node("align left").horizontal_group(vec![self, stretch_x]),
                AlignX::Center => node("align center (x)").horizontal_group(vec![stretch_x.clone(), self, stretch_x]),
                AlignX::Right => node("align right").horizontal_group(vec![stretch_x, self]),
            }
        };
        let stretch_y = node("stretch y").height_stretch().width(0.0);
        match y {
            AlignY::Top => node("align top").vertical_group(vec![row, stretch_y]),
            AlignY::Center => node("align center (y)").vertical_group(vec![stretch_y.clone(), row, stretch_y]),
            AlignY::Bottom => node("align bottom").vertical_group(vec![stretch_y, row]),
        }
    }
}
