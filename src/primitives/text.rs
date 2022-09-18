use macroquad::color::Color;
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::TextDimensions;
use macroquad::text::draw_text;
use macroquad::text::measure_text;

use crate::core::Ctx;
use crate::core::Element;
use crate::core::Phase;

#[derive(Debug, Clone)]
pub struct Text {
    pub value: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
    pub shadow: Option<Vec<(Vec2, Color)>>
}

impl From<(f32, Color)> for TextStyle {
    fn from((font_size, color): (f32, Color)) -> Self {
        TextStyle { font_size, color, shadow: None }
    }
}

impl<Event> Element<Event> for Text {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw { .. } => {
                let text = self.value.as_str();
                let size = self.measure_self();
                let pos = vec2(ctx.area.x, ctx.area.y + size.offset_y + 0.2 * self.style.font_size);
                draw_text(text, pos.x, pos.y, self.style.font_size, self.style.color);
                let draw = |pos: Vec2, color: Color| {
                    draw_text(text, pos.x, pos.y, self.style.font_size, color);
                };
                if let Some(shadow) = &self.style.shadow {
                    for (offset, color) in shadow.iter().cloned() {
                        draw(pos + offset, color);
                    }
                }
                draw(pos, self.style.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}

impl Text {
    pub fn measure_self(&self) -> TextDimensions {
        measure_text(self.value.as_str(), None, self.style.font_size as u16, 1.0)
    }
}
