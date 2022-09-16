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
pub struct Width(pub Dimension);

#[derive(Debug, Copy, Clone)]
pub struct Height(pub Dimension);

impl<Event> Element<Event> for Width {}

impl<Event> Element<Event> for Height {}

#[derive(Debug, Copy, Clone)]
pub enum Dimension {
    Fixed(f32),
    Stretch { fixed_part: f32 },
    RemoveStretch,
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
            Layout::Vertical => self.do_layout(&ctx, DimensionKey::Vertical),
            Layout::Horizontal => self.do_layout(&ctx, DimensionKey::Horizontal),
        }
    }
}

impl<Event: Clone + Debug + 'static> Group<Event> {
    fn do_layout(&self, ctx: &Ctx<Event>, dimension: DimensionKey) {
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
                    CalculatedSize::Fixed(value) => total_size += value,
                    CalculatedSize::Stretch { fixed_part } => {
                        total_size += fixed_part;
                        stretch_count += 1;
                    }
                }
            }
            let forward_area_size = match dimension {
                DimensionKey::Horizontal => ctx.area.w,
                DimensionKey::Vertical => ctx.area.h,
            };
            (forward_area_size - total_size) / stretch_count as f32
        };
        let mut offset = match dimension {
            DimensionKey::Horizontal => ctx.area.x,
            DimensionKey::Vertical => ctx.area.y,
        };
        for (i, (child, size)) in sized_children.iter().copied().enumerate() {
            let size = match size {
                CalculatedSize::Fixed(value) => value,
                CalculatedSize::Stretch { fixed_part } => fixed_part + stretch_size,
            };
            let mut ctx = ctx.step_down(UiPathStep::Index(i));
            if let Some(name) = child.get_name() {
                ctx = ctx.step_down(UiPathStep::Name(name));
            }
            child.do_phase(ctx
                .clone_with(|ctx| ctx.area = Rect::new(
                    match dimension {
                        DimensionKey::Horizontal => offset,
                        DimensionKey::Vertical => ctx.area.x,
                    },
                    match dimension {
                        DimensionKey::Horizontal => ctx.area.y,
                        DimensionKey::Vertical => offset,
                    },
                    match dimension {
                        DimensionKey::Horizontal => size,
                        DimensionKey::Vertical => ctx.area.w,
                    },
                    match dimension {
                        DimensionKey::Horizontal => ctx.area.h,
                        DimensionKey::Vertical => size,
                    },
                )));
            offset += size;
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum DimensionKey {
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone)]
enum CalculatedSize {
    Fixed(f32),
    Stretch { fixed_part: f32 },
}

fn calc_size_dimension<Event>(
    node: &Node<Event>,
    dimension: DimensionKey,
    ctx: &Ctx<Event>,
) -> CalculatedSize
    where Event: Clone + Debug + 'static
{
    let mut ctx = ctx.clone();
    if let Some(name) = node.get_name() {
        ctx = ctx.step_down(UiPathStep::Name(name));
    }
    let dimension_value = match dimension {
        DimensionKey::Horizontal => node.get::<Width>().map(|it| it.0),
        DimensionKey::Vertical => node.get::<Height>().map(|it| it.0),
    };
    let flow = match dimension_value {
        None => Flow::Calculate(CalculateFlow::AsIs),
        Some(size) => match size {
            Dimension::Fixed(size) => Flow::Propagate(CalculatedSize::Fixed(size)),
            Dimension::Stretch { fixed_part } => Flow::Propagate(CalculatedSize::Stretch { fixed_part }),
            Dimension::RemoveStretch => Flow::Calculate(CalculateFlow::RemoveStretch),
        },
    };
    enum CalculateFlow {
        AsIs,
        RemoveStretch,
    }
    enum Flow {
        Propagate(CalculatedSize),
        Calculate(CalculateFlow),
    }
    match flow {
        Flow::Propagate(size) => size,
        Flow::Calculate(sub_flow) => {
            match node.get::<Group<Event>>() {
                None => panic!(
                    "failed to resolve {:?} size of '{}' ({})",
                    dimension,
                    node.get_name().unwrap_or("unknown"),
                    ctx.backtrace(),
                ),
                Some(group) => {
                    let merge_strategy = match dimension {
                        DimensionKey::Horizontal => match group.layout {
                            Layout::Layered => CalculatedSize::max,
                            Layout::Vertical => CalculatedSize::max,
                            Layout::Horizontal => CalculatedSize::sum,
                        }
                        DimensionKey::Vertical => match group.layout {
                            Layout::Layered => CalculatedSize::max,
                            Layout::Vertical => CalculatedSize::sum,
                            Layout::Horizontal => CalculatedSize::max,
                        }
                    };
                    let final_size = group.children.iter().enumerate()
                        .map(|(i, it)| calc_size_dimension(
                            it, dimension,
                            &ctx.step_down(UiPathStep::Index(i)),
                        ))
                        .reduce(merge_strategy)
                        .unwrap_or(CalculatedSize::Stretch { fixed_part: 0.0 });
                    match sub_flow {
                        CalculateFlow::AsIs => final_size,
                        CalculateFlow::RemoveStretch => CalculatedSize::Fixed(final_size.get_fixed_part()),
                    }
                }
            }
        }
    }
}

impl CalculatedSize {
    fn sum(a: CalculatedSize, b: CalculatedSize) -> CalculatedSize {
        let fixed_part = a.get_fixed_part() + b.get_fixed_part();
        if a.is_stretch() || b.is_stretch() {
            CalculatedSize::Stretch { fixed_part }
        } else {
            CalculatedSize::Fixed(fixed_part)
        }
    }

    fn max(a: CalculatedSize, b: CalculatedSize) -> CalculatedSize {
        let fixed_part = a.get_fixed_part().max(b.get_fixed_part());
        if a.is_stretch() || b.is_stretch() {
            CalculatedSize::Stretch { fixed_part }
        } else {
            CalculatedSize::Fixed(fixed_part)
        }
    }

    fn is_stretch(&self) -> bool {
        match self {
            CalculatedSize::Fixed(_) => false,
            CalculatedSize::Stretch { .. } => true,
        }
    }

    fn get_fixed_part(&self) -> f32 {
        *match self {
            CalculatedSize::Fixed(value) => value,
            CalculatedSize::Stretch { fixed_part } => fixed_part
        }
    }
}