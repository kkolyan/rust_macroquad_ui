use std::fmt::Debug;
use crate::primitives::group::Group;
use crate::primitives::{height, height_stretch, horizontal_group, vertical_group, width, width_stretch};
use crate::primitives::node::{Node, node};

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

pub fn align<Event: 'static + Clone + Debug>(x: AlignX, y: AlignY, target: Node<Event>) -> Group<Event> {
    let row = {
        let stretch_x = node("stretch x")
            .set(width_stretch())
            .set(height(0.0));
        match x {
            AlignX::Left => node("align left").set(horizontal_group(vec![target, stretch_x])),
            AlignX::Center => node("align center (x)").set(horizontal_group(vec![stretch_x.clone(), target, stretch_x])),
            AlignX::Right => node("align right").set(horizontal_group(vec![stretch_x, target])),
        }
    };
    let stretch_y = node("stretch y").set(height_stretch()).set(width(0.0));
    horizontal_group(vec![
        match y {
            AlignY::Top => node("align top").set(vertical_group(vec![row, stretch_y])),
            AlignY::Center => node("align center (y)").set(vertical_group(vec![stretch_y.clone(), row, stretch_y])),
            AlignY::Bottom => node("align bottom").set(vertical_group(vec![stretch_y, row])),
        }
    ])
}
