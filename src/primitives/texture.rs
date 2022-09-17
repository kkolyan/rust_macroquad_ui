use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::draw_texture_ex;
use macroquad::texture::{DrawTextureParams, Texture2D};

use crate::core::Ctx;
use crate::core::Element;
use crate::core::Phase;

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub value: Texture2D,
    pub color: Color,
    pub dst_size: Option<Vec2>,
}

impl<Event> Element<Event> for Texture {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw { .. } => {
                let texture = self.value;
                let size = Vec2::new(texture.width(), texture.height());
                let pos = Vec2::new(ctx.area.x, ctx.area.y);
                draw_texture_ex(texture, pos.x, pos.y, self.color, DrawTextureParams {
                    dest_size: self.dst_size.or_else(|| Some(size)),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                });
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}

pub fn texture<T: Into<Texture>>(t: T) -> Texture {
    t.into()
}