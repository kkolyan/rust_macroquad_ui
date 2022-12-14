use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use crate::core::{Ctx, Element, Phase};

#[derive(Debug, Copy, Clone)]
pub struct ColorFill {
    color: Color,
}

impl From<Color> for ColorFill {
    fn from(color: Color) -> Self {
        ColorFill { color }
    }
}

impl<Event> Element<Event> for ColorFill {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw { .. } => {
                draw_rectangle(ctx.area.x, ctx.area.y, ctx.area.w, ctx.area.h, self.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}
