use std::fmt::Debug;
use macroquad::math::Rect;
use crate::core::{Ctx, Phase, Element};
use crate::elements::node::Node;

#[derive(Debug, Copy, Clone)]
pub enum Layout {
    Layered,
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone)]
pub struct Width(pub f32);

pub trait WidthFactory<Event> {
    fn width(self, value: f32) -> Self;
}

impl <Event> WidthFactory<Event> for Node<Event> {
    fn width(self, value: f32) -> Self {
        self.imbue(Width(value))
    }
}

impl<Event> Element<Event> for Width {}

#[derive(Debug, Copy, Clone)]
pub struct Height(pub f32);

impl<Event> Element<Event> for Height {}

pub trait HeightFactory<Event> {
    fn height(self, value: f32) -> Self;
}

impl <Event> HeightFactory<Event> for Node<Event> {
    fn height(self, value: f32) -> Self {
        self.imbue(Height(value))
    }
}

#[derive(Debug, Clone)]
pub struct Group<Event> {
    layout: Layout,
    children: Vec<Node<Event>>,
}

pub trait GroupFactory<Event> {
    fn group(self, layout: Layout, children: Vec<Node<Event>>) -> Self;
}

impl <Event: 'static +  Clone + Debug> GroupFactory<Event> for Node<Event> {
    fn group(self, layout: Layout, children: Vec<Node<Event>>) -> Self {
        self.imbue(Group::new(layout, children))
    }
}

impl<Event> Group<Event> {
    pub fn new(layout: Layout, children: Vec<Node<Event>>) -> Self {
        Group { layout, children }
    }
}

impl<Event: Clone> Element<Event> for Group<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match self.layout {
            Layout::Layered => {
                for child in &self.children {
                    child.do_phase(ctx.clone());
                }
            }
            Layout::Vertical => {
                let mut offset = ctx.area.y;
                for child in &self.children {
                    let size = child.components.get::<Height>()
                        .expect("Height required for layout items")
                        .0.clone();
                    child.do_phase(ctx.clone_with(|ctx| ctx.area = Rect::new(
                        ctx.area.x.clone(),
                        offset.clone(),
                        ctx.area.w.clone(),
                        size,
                    )));
                    offset += size;
                }
            }
            Layout::Horizontal => {
                let mut offset = ctx.area.x;
                for child in &self.children {
                    let size = child.components.get::<Width>()
                        .expect(" required for layout items")
                        .0.clone();
                    child.do_phase(ctx.clone_with(|ctx| ctx.area = Rect::new(
                        offset.clone(),
                        ctx.area.y.clone(),
                        size,
                        ctx.area.h.clone(),
                    )));
                    offset += size;
                }
            }
        }
    }
}
