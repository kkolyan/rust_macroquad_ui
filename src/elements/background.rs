use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use crate::core::{Ctx, Element, Phase};
use crate::elements::node::{Node, NodePlugin};

#[derive(Debug, Copy, Clone)]
pub struct Background {
    color: Color,
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background { color }
    }
}

pub trait BackgroundFactory<Event> {
    fn background_from_color(self, color: Color) -> Self;
}

impl <Event: Clone> BackgroundFactory<Event> for Node<Event> {
    fn background_from_color(self, color: Color) -> Self {
        self.add_component(Background::from(color))
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

impl<Event: Clone> NodePlugin<Event> for Background {}
