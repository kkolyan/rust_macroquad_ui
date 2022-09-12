use std::fmt::Debug;
use macroquad::math::Rect;
use crate::core::{Ctx, Phase, Element, UiPathStep};
use crate::elements::name::{Name};
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

impl<Event: Clone + Debug + 'static> Element<Event> for Group<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match self.layout {
            Layout::Layered => {
                for child in &self.children {
                    child.do_phase(ctx.clone());
                }
            }
            Layout::Vertical => self.do_layout(&ctx, Dimension::Vertical),
            Layout::Horizontal => self.do_layout(&ctx, Dimension::Horizontal),
        }
    }
}

impl<Event: Clone + Debug + 'static> Group<Event> {
    fn do_layout(&self, ctx: &Ctx<Event>, dimension: Dimension) {
        let sized_children: Vec<_> = self.children.iter().enumerate()
            .map(|(i, it)| (it, calc_size_dimension(
                it,
                dimension,
                &ctx.step_down(UiPathStep::Index(i)),
            )))
            .collect();
        let stretch_size = {
            let mut total_size = 0.0;
            let mut stretch_count = 0;
            for (_, size) in sized_children.iter().copied() {
                match size {
                    Size1D::Fixed(value) => total_size += value,
                    Size1D::Stretch => stretch_count += 1,
                }
            }
            let forward_area_size = match dimension {
                Dimension::Horizontal => ctx.area.w,
                Dimension::Vertical => ctx.area.h,
            };
            (forward_area_size - total_size) / stretch_count as f32
        };
        let mut offset = match dimension {
            Dimension::Horizontal => ctx.area.x,
            Dimension::Vertical => ctx.area.y,
        };
        for (i, (child, size)) in sized_children.iter().copied().enumerate() {
            let size = match size {
                Size1D::Fixed(value) => value,
                Size1D::Stretch => stretch_size,
            };
            child.do_phase(ctx
                .step_down(UiPathStep::Index(i))
                .step_down(UiPathStep::extract_path(child, "<node>"))
                .clone_with(|ctx| ctx.area = Rect::new(
                    match dimension {
                        Dimension::Horizontal => offset,
                        Dimension::Vertical => ctx.area.x,
                    },
                    match dimension {
                        Dimension::Horizontal => ctx.area.y,
                        Dimension::Vertical => offset,
                    },
                    match dimension {
                        Dimension::Horizontal => size,
                        Dimension::Vertical => ctx.area.w,
                    },
                    match dimension {
                        Dimension::Horizontal => ctx.area.h,
                        Dimension::Vertical => size,
                    },
                )));
            offset += size;
        }
    }
}

enum MergeMode {
    Max,
    Sum,
}

#[derive(Copy, Clone, Debug)]
enum Dimension {
    Horizontal,
    Vertical,
}

fn calc_size_dimension<Event>(
    node: &Node<Event>,
    dimension: Dimension,
    ctx: &Ctx<Event>,
) -> Size1D
    where Event: Clone + Debug + 'static
{
    let ctx = ctx.step_down(UiPathStep::extract_path(node, "<node>"));
    let dimension_value = match dimension {
        Dimension::Horizontal => node.components.get::<Width>().map(|it| it.0),
        Dimension::Vertical => node.components.get::<Height>().map(|it| it.0),
    };
    match dimension_value {
        Some(size) => size,
        None => match node.components.get::<Group<Event>>() {
            None => panic!(
                "failed to resolve {:?} size of '{}' ({})",
                dimension,
                node.components.get::<Name>().map(|it| it.0).unwrap_or("unknown"),
                ctx.backtrace(),
            ),
            Some(group) => {
                let merge_strategy = match dimension {
                    Dimension::Horizontal => match group.layout {
                        Layout::Layered => Size1D::max,
                        Layout::Vertical => Size1D::max,
                        Layout::Horizontal => Size1D::sum,
                    }
                    Dimension::Vertical => match group.layout {
                        Layout::Layered => Size1D::max,
                        Layout::Vertical => Size1D::sum,
                        Layout::Horizontal => Size1D::max,
                    }
                };
                group.children.iter().enumerate()
                    .map(|(i, it)| calc_size_dimension(
                        it, dimension,
                        &ctx.step_down(UiPathStep::Index(i)),
                    ))
                    .reduce(merge_strategy)
                    .unwrap_or(Size1D::Stretch)
            }
        },
    }
}

impl Size1D {

    fn sum(a: Size1D, b: Size1D) -> Size1D {
        match a {
            Size1D::Fixed(a) => match b {
                Size1D::Fixed(b) => Size1D::Fixed(a + b),
                Size1D::Stretch => Size1D::Stretch,
            }
            Size1D::Stretch => Size1D::Stretch
        }
    }

    fn max(a: Size1D, b: Size1D) -> Size1D {
        match a {
            Size1D::Fixed(a) => match b {
                Size1D::Fixed(b) => Size1D::Fixed(a.max(b)),
                Size1D::Stretch => Size1D::Stretch
            }
            Size1D::Stretch => Size1D::Stretch,
        }
    }

}