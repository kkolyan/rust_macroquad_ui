use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use crate::core::{Ctx, Element, Phase};

#[derive(Debug, Copy, Clone)]
pub struct Background {
    color: Color,
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background { color }
    }
}

impl<Event> Element<Event> for Background {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw => {
                draw_rectangle(ctx.area.x, ctx.area.y, ctx.area.w, ctx.area.h, self.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}
