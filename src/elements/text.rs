use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::TextDimensions;
use macroquad::text::draw_text;
use macroquad::text::measure_text;

use crate::core::{Ctx, Dimension, Element, GetSizeCtx, Phase, UiPathStep};
use crate::elements::node::Node;

use crate::elements::common::AlignY;
use crate::elements::common::AlignX;
use crate::elements::group::Size1D;

#[derive(Debug, Clone)]
pub struct Text {
    value: String,
    font_size: f32,
    color: Color,
    align_x: AlignX,
    align_y: AlignY,
}

pub trait TextFactory<Event> {
    fn text(self, value: String, font_size: f32, color: Color, align_x: AlignX, align_y: AlignY) -> Self;
}

impl <Event> TextFactory<Event> for Node<Event> {
    fn text(self, value: String, font_size: f32, color: Color, align_x: AlignX, align_y: AlignY) -> Self {
        self.add_component(Text {
            value,
            font_size,
            color,
            align_x,
            align_y
        })
    }
}

impl<Event> Element<Event> for Text {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw => {
                let text = self.value.as_str();
                let size = self.measure_self();
                let pos = Vec2::new(
                    match self.align_x {
                        AlignX::Left => ctx.area.x,
                        AlignX::Center => ctx.area.x + (ctx.area.w - size.width) * 0.5,
                        AlignX::Right => ctx.area.x + ctx.area.w - size.width,
                    },
                    match self.align_y {
                        AlignY::Top => ctx.area.y + size.offset_y,
                        AlignY::Center => ctx.area.y + (ctx.area.h - size.height + size.offset_y) * 0.5,
                        AlignY::Bottom => ctx.area.y + ctx.area.h - size.height,
                    },
                );
                draw_text(text, pos.x, pos.y, self.font_size, self.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }

    fn get_size(&self, dim: Dimension, ctx: GetSizeCtx) -> Option<Size1D> {
        let size = self.measure_self();
        Some(Size1D::Fixed(match dim {
            Dimension::X => size.width,
            Dimension::Y => size.height,
        }))
    }
}

impl Text {
    fn measure_self(&self) -> TextDimensions {
        measure_text(self.value.as_str(), None, self.font_size as u16, 1.0)
    }
}
