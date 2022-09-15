use std::fmt::Debug;
use macroquad::text::{measure_text, TextDimensions};
use crate::basic_composites::align::{align, AlignY};
use crate::basic_composites::align::AlignX;
use crate::primitives::{height, height_no_stretch, horizontal_group, text, width, width_no_stretch};
use crate::primitives::node::{node, Node};
use crate::primitives::text::TextStyle;

#[derive(Debug, Copy, Clone)]
pub struct LabelStyle {
    pub text: TextStyle,
    pub align: (AlignX, AlignY),
}

impl From<TextStyle> for LabelStyle {
    fn from(style: TextStyle) -> Self {
        LabelStyle { text: style, align: (AlignX::Left, AlignY::Top) }
    }
}

impl From<(TextStyle, AlignX, AlignY)> for LabelStyle {
    fn from(value: (TextStyle, AlignX, AlignY)) -> Self {
        LabelStyle { text: value.0, align: (value.1, value.2) }
    }
}

pub fn label<Event: Clone + Debug + 'static, T: Into<String>, S: Into<LabelStyle>>(t: T, style: S) -> Node<Event> {
    let t = t.into();
    let style = style.into();
    let size = measure_self(t.as_str(), style.text.font_size);
    let label = node("label")
        .set(text(t, style.text))
        .set(width(size.width))
        .set(height(style.text.font_size));
    node("label")
        .set(align(style.align.0, style.align.1, node("label")
            .set(horizontal_group(vec![
                label
            ]))))
        .set(width_no_stretch())
        .set(height_no_stretch())
}

fn measure_self(text: &str, font_size: f32) -> TextDimensions {
    measure_text(text, None, font_size as u16, 1.0)
}
