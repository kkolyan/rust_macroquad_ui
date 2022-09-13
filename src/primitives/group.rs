use std::fmt::Debug;
use macroquad::math::Rect;
use crate::core::Element;
use crate::core::Ctx;
use crate::core::UiPathStep;
use crate::primitives::node::Node;

#[derive(Debug, Copy, Clone)]
pub enum Layout {
    Layered,
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone)]
pub struct Width(pub Size);

#[derive(Debug, Copy, Clone)]
pub struct Height(pub Size);

impl<Event> Element<Event> for Width {}
impl<Event> Element<Event> for Height {}

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Fixed(f32),
    Stretch { fixed_part: f32 },
    RemoveStretch,
}

#[derive(Debug, Copy, Clone)]
enum FinalSize {
    Fixed(f32),
    Stretch { fixed_part: f32 },
}

#[derive(Debug, Clone)]
pub struct Group<Event> {
    layout: Layout,
    children: Vec<Node<Event>>,
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
                    FinalSize::Fixed(value) => total_size += value,
                    FinalSize::Stretch { fixed_part } => {
                        total_size += fixed_part;
                        stretch_count += 1;
                    },
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
                FinalSize::Fixed(value) => value,
                FinalSize::Stretch {fixed_part} => fixed_part + stretch_size,
            };
            child.do_phase(ctx
                .step_down(UiPathStep::Index(i))
                .step_down(UiPathStep::Name(child.name.unwrap_or("<node>")))
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

#[derive(Copy, Clone, Debug)]
enum Dimension {
    Horizontal,
    Vertical,
}

fn calc_size_dimension<Event>(
    node: &Node<Event>,
    dimension: Dimension,
    ctx: &Ctx<Event>,
) -> FinalSize
    where Event: Clone + Debug + 'static
{
    let ctx = ctx.step_down(UiPathStep::Name(node.name.unwrap_or("<node>")));
    let dimension_value = match dimension {
        Dimension::Horizontal => node.components.get::<Width>().map(|it| it.0),
        Dimension::Vertical => node.components.get::<Height>().map(|it| it.0),
    };
    let flow = match dimension_value {
        None => Flow::Calculate(CalculateFlow::AsIs),
        Some(size) => match size {
            Size::Fixed(size) => Flow::Propagate(FinalSize::Fixed(size)),
            Size::Stretch { fixed_part } => Flow::Propagate(FinalSize::Stretch { fixed_part}),
            Size::RemoveStretch => Flow::Calculate(CalculateFlow::RemoveStretch),
        },
    };
    enum CalculateFlow {
        AsIs,
        RemoveStretch,
    }
    enum Flow {
        Propagate(FinalSize),
        Calculate(CalculateFlow),
    }
    match flow {
        Flow::Propagate(size) => size,
        Flow::Calculate(sub_flow) => {
            match node.components.get::<Group<Event>>() {
                None => panic!(
                    "failed to resolve {:?} size of '{}' ({})",
                    dimension,
                    node.name.unwrap_or("unknown"),
                    ctx.backtrace(),
                ),
                Some(group) => {
                    let merge_strategy = match dimension {
                        Dimension::Horizontal => match group.layout {
                            Layout::Layered => FinalSize::max,
                            Layout::Vertical => FinalSize::max,
                            Layout::Horizontal => FinalSize::sum,
                        }
                        Dimension::Vertical => match group.layout {
                            Layout::Layered => FinalSize::max,
                            Layout::Vertical => FinalSize::sum,
                            Layout::Horizontal => FinalSize::max,
                        }
                    };
                    let final_size = group.children.iter().enumerate()
                        .map(|(i, it)| calc_size_dimension(
                            it, dimension,
                            &ctx.step_down(UiPathStep::Index(i)),
                        ))
                        .reduce(merge_strategy)
                        .unwrap_or(FinalSize::Stretch { fixed_part: 0.0 });
                    match sub_flow {
                        CalculateFlow::AsIs => final_size,
                        CalculateFlow::RemoveStretch => FinalSize::Fixed(final_size.get_fixed_part()),
                    }
                }
            }
        }
    }
}

impl FinalSize {
    fn sum(a: FinalSize, b: FinalSize) -> FinalSize {
        let fixed_part = a.get_fixed_part() + b.get_fixed_part();
        if a.is_stretch() || b.is_stretch() {
            FinalSize::Stretch { fixed_part }
        } else {
            FinalSize::Fixed(fixed_part)
        }
    }

    fn max(a: FinalSize, b: FinalSize) -> FinalSize {
        let fixed_part = a.get_fixed_part().max(b.get_fixed_part());
        if a.is_stretch() || b.is_stretch() {
            FinalSize::Stretch { fixed_part }
        } else {
            FinalSize::Fixed(fixed_part)
        }
    }

    fn is_stretch(&self) -> bool {
        match self {
            FinalSize::Fixed(_) => false,
            FinalSize::Stretch { .. } => true,
        }
    }

    fn get_fixed_part(&self) -> f32 {
        *match self {
            FinalSize::Fixed(value) => value,
            FinalSize::Stretch { fixed_part } => fixed_part
        }
    }
}