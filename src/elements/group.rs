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
pub struct Width(pub Size1D);


#[derive(Debug, Copy, Clone)]
pub enum Size1D {
    Fixed(f32),
    Stretch,
}

pub trait WidthFactory<Event> {
    fn width(self, value: f32) -> Self;
    fn width_stretch(self) -> Self;
}

impl<Event> WidthFactory<Event> for Node<Event> {
    fn width(self, value: f32) -> Self {
        self.add_component(Width(Size1D::Fixed(value)))
    }

    fn width_stretch(self) -> Self {
        self.add_component(Width(Size1D::Stretch))
    }
}

impl<Event> Element<Event> for Width {}

#[derive(Debug, Copy, Clone)]
pub struct Height(pub Size1D);

impl<Event> Element<Event> for Height {}

pub trait HeightFactory<Event> {
    fn height(self, value: f32) -> Self;
    fn height_stretch(self) -> Self;
}

impl<Event> HeightFactory<Event> for Node<Event> {
    fn height(self, value: f32) -> Self {
        self.add_component(Height(Size1D::Fixed(value)))
    }

    fn height_stretch(self) -> Self {
        self.add_component(Height(Size1D::Stretch))
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

impl<Event: 'static + Clone + Debug> GroupFactory<Event> for Node<Event> {
    fn group(self, layout: Layout, children: Vec<Node<Event>>) -> Self {
        self.add_component(Group::new(layout, children))
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
                let stretch_size = {
                    let mut total_size = 0.0;
                    let mut stretch_count = 0;
                    for child in &self.children {
                        let width = child.components.get::<Height>()
                            .expect("Height required for layout items")
                            .0;
                        match width {
                            Size1D::Fixed(value) => total_size += value,
                            Size1D::Stretch => stretch_count += 1,
                        }
                    }
                    (ctx.area.h - total_size) / stretch_count as f32
                };
                let mut offset = ctx.area.y;
                for child in &self.children {
                    let width = child.components.get::<Height>()
                        .expect("Height required for layout items")
                        .0;
                    let size = match width {
                        Size1D::Fixed(value) => value,
                        Size1D::Stretch => stretch_size,
                    };
                    child.do_phase(ctx.clone_with(|ctx| ctx.area = Rect::new(
                        ctx.area.x,
                        offset,
                        ctx.area.w,
                        size,
                    )));
                    offset += size;
                }
            }
            Layout::Horizontal => {
                let stretch_size = {
                    let mut total_size = 0.0;
                    let mut stretch_count = 0;
                    for child in &self.children {
                        let width = child.components.get::<Width>()
                            .expect("Width required for layout items")
                            .0;
                        match width {
                            Size1D::Fixed(value) => total_size += value,
                            Size1D::Stretch => stretch_count += 1,
                        }
                    }
                    (ctx.area.w - total_size) / stretch_count as f32
                };
                let mut offset = ctx.area.x;
                for child in &self.children {
                    let width = child.components.get::<Width>()
                        .expect(" required for layout items")
                        .0;
                    let size = match width {
                        Size1D::Fixed(value) => value,
                        Size1D::Stretch => stretch_size,
                    };
                    child.do_phase(ctx.clone_with(|ctx| ctx.area = Rect::new(
                        offset,
                        ctx.area.y,
                        size,
                        ctx.area.h,
                    )));
                    offset += size;
                }
            }
        }
    }
}
