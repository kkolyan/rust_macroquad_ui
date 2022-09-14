use std::fmt::Debug;
use macroquad::text::{measure_text, TextDimensions};
use crate::basic_composites::align::{AlignX, AlignY, FluentAlign};
use crate::fluent_primitives::FluentPrimitives;
use crate::primitives::node::{Node, node};
use crate::primitives::text::{TextStyle};

#[derive(Debug, Copy, Clone)]
pub struct LabelStyle {
    pub text: TextStyle,
    pub align: (AlignX, AlignY),
}

pub trait FluentLabel {
    fn label<T: Into<String>, S: Into<LabelStyle>>(self, text: T, style: S) -> Self;
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

impl<Event: Clone + Debug + 'static> FluentLabel for Node<Event> {
    fn label<T: Into<String>, S: Into<LabelStyle>>(self, text: T, style: S) -> Self {
        let text = text.into();
        let style = style.into();
        let size = measure_self(text.as_str(), style.text.font_size);
        let label = node("label")
            .text(text, style.text)
            .width(size.width)
            .height(style.text.font_size);
        self
            .horizontal_group(vec![label])
            .wrap_align(style.align.0, style.align.1)
            .width_no_stretch()
            .height_no_stretch()
    }
}

fn measure_self(text: &str, font_size: f32) -> TextDimensions {
    measure_text(text, None, font_size as u16, 1.0)
}
