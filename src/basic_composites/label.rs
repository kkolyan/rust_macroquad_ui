use std::fmt::Debug;
use macroquad::text::{measure_text, TextDimensions};
use crate::primitives::{height, text, width};
use crate::primitives::node::{node, Node};
use crate::primitives::text::TextStyle;


pub fn label<Event: Clone + Debug + 'static, T: Into<String>, S: Into<TextStyle>>(t: T, style: S) -> Node<Event> {
    let t = t.into();
    let style = style.into();
    let size = measure_self(t.as_str(), style.font_size);
    node().name("label")
        .set(text(t, style))
        .set(width(size.width))
        .set(height(style.font_size))
}

fn measure_self(text: &str, font_size: f32) -> TextDimensions {
    measure_text(text, None, font_size as u16, 1.0)
}
