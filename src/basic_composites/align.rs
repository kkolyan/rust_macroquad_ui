use std::fmt::Debug;
use crate::primitives::{height, height_stretch, horizontal_content, vertical_content, width, width_stretch};
use crate::primitives::node::{Node, node, NodePadding};

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

#[derive(Debug, Copy, Clone)]
pub struct Align {
    x: AlignX,
    y: AlignY,
}

impl<Event: Clone + Debug + 'static> NodePadding<Event> for Align {
    fn expand_padding(&self, content: Node<Event>) -> Node<Event> {
        let row = {
            let stretch_x = node().name("stretch x")
                .set(width_stretch())
                .set(height(0.0));
            match self.x {
                AlignX::Left => node().name("align left").set(horizontal_content(vec![content, stretch_x])),
                AlignX::Center => node().name("align center (x)").set(horizontal_content(vec![stretch_x.clone(), content, stretch_x])),
                AlignX::Right => node().name("align right").set(horizontal_content(vec![stretch_x, content])),
            }
        };
        let stretch_y = node().name("stretch y").set(height_stretch()).set(width(0.0));
        node()
            .name("align")
            .set(horizontal_content(vec![
                match self.y {
                    AlignY::Top => node().name("align top").set(vertical_content(vec![row, stretch_y])),
                    AlignY::Center => node().name("align center (y)").set(vertical_content(vec![stretch_y.clone(), row, stretch_y])),
                    AlignY::Bottom => node().name("align bottom").set(vertical_content(vec![stretch_y, row])),
                }
            ]))
    }
}

pub fn align(x: AlignX, y: AlignY) -> Align {
    Align { x, y }
}
