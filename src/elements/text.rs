use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::TextDimensions;
use macroquad::text::draw_text;
use macroquad::text::measure_text;

use crate::core::Ctx;
use crate::core::Element;
use crate::core::Phase;
use crate::elements::group::HeightFactory;
use crate::elements::group::WidthFactory;
use crate::elements::node::Node;

#[derive(Debug, Clone)]
pub struct Text {
    value: String,
    style: TextStyle,
}

pub trait TextFactory<Event> {
    fn text<S: ToText>(self, value: S, style: TextStyle) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
}

impl <Event: Clone> TextFactory<Event> for Node<Event> {
    fn text<S: ToText>(self, value: S, style: TextStyle) -> Self {
        let text = Text {
            value: value.to_text(),
            style,
        };
        let size = text.measure_self();
        self.add_component(text)
            .width(size.width)
            .height(style.font_size)// because size.y varies depends on the presence of letters like "p"
    }
}

impl<Event> Element<Event> for Text {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw => {
                let text = self.value.as_str();
                let size = self.measure_self();
                let pos = Vec2::new(ctx.area.x, ctx.area.y + size.offset_y + 0.125 * self.style.font_size);
                draw_text(text, pos.x, pos.y, self.style.font_size, self.style.color);
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}

impl Text {
    fn measure_self(&self) -> TextDimensions {
        measure_text(self.value.as_str(), None, self.style.font_size as u16, 1.0)
    }
}

pub trait ToText {
    fn to_text(self) -> String;
}

impl ToText for String {
    fn to_text(self) -> String {
        self
    }
}

impl ToText for &str {
    fn to_text(self) -> String {
        self.to_owned()
    }
}