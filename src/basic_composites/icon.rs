use std::fmt::Debug;
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::texture::Texture2D;
use crate::basic_composites::align::{align, AlignY};
use crate::basic_composites::align::AlignX;
use crate::basic_composites::no_stretch::no_stretch;
use crate::basic_composites::no_stretch::NoStretchMode::Both;
use crate::primitives::{height, width};
use crate::primitives::node::{node, Node};
use crate::primitives::texture::{Texture, texture};

#[derive(Debug, Clone, Copy)]
pub struct Icon {
    pub texture: Texture,
    pub align: (AlignX, AlignY),
}

impl From<(Texture2D, Color, Vec2)> for Icon {
    fn from((tex, color, size): (Texture2D, Color, Vec2)) -> Self {
        Icon {
            texture: Texture {
                value: tex,
                color,
                dst_size: Some(size)
            },
            align: (AlignX::Center, AlignY::Center)
        }
    }
}


pub fn icon<Event, T>(t: T) -> Node<Event>
    where Event: Clone + Debug + 'static,
          T: Into<Icon>
{
    let t = t.into();
    let size = t.texture.dst_size.unwrap_or_else(|| Vec2::new(t.texture.value.width(), t.texture.value.height()));
    node().name("icon")
        .pad(no_stretch(Both))
        .pad(align(t.align.0, t.align.1))
        .set(texture(t.texture))
        .set(width(size.x))
        .set(height(size.y))
}
