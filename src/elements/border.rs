use macroquad::color::Color;
use macroquad::shapes::draw_rectangle_lines;
use crate::core::{Ctx, Element, Phase};

#[derive(Debug, Copy, Clone)]
pub struct Border {
    thickness: f32,
    color: Color,
}

impl<Event> Element<Event> for Border {
    fn do_phase(&self, ctx: Ctx<Event>)  {
        match ctx.phase {
            Phase::Draw => {
                draw_rectangle_lines(ctx.area.x, ctx.area.y, ctx.area.w, ctx.area.y, self.thickness, self.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}
