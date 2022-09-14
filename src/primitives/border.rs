use macroquad::color::Color;
use macroquad::shapes::draw_rectangle_lines;
use crate::core::{Ctx, Element, Phase};
use crate::primitives::node2::{NodeChain, NodeComponent};

#[derive(Debug, Copy, Clone)]
pub struct Border {
    thickness: f32,
    color: Color,
}

impl<Event> NodeComponent<Event> for Border {
    fn do_phase_(&self, ctx: Ctx<Event>, next: NodeChain<Event>) {
        match ctx.phase {
            Phase::Draw => {
                draw_rectangle_lines(ctx.area.x, ctx.area.y, ctx.area.w, ctx.area.y, self.thickness, self.color);
            }
            Phase::CollectEvents { .. } => {}
        }
        next.do_phase(ctx)
    }
}
