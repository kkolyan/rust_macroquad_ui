use std::fmt::Debug;

use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::texture::Texture2D;

use crate::primitives::{height, width};
use crate::primitives::node::{node, Node};
use crate::primitives::texture::{Texture, texture};

pub enum IconSize {
    Source,
    Fixed(f32, f32),
}

pub fn icon<Event>(image: Texture2D, color: Color, size: IconSize) -> Node<Event>
    where Event: Clone + Debug + 'static
{
    let t = Texture {
        value: image,
        color,
        dst_size: match size {
            IconSize::Source => None,
            IconSize::Fixed(x, y) => Some(Vec2::new(x, y)),
        },
    };
    let size = t.dst_size.unwrap_or_else(|| Vec2::new(t.value.width(), t.value.height()));
    node().name("icon")
        .set(texture(t))
        .set(width(size.x))
        .set(height(size.y))
}
