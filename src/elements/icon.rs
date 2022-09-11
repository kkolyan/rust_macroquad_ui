use macroquad::color::Color;
use macroquad::math::{Rect, Vec2};
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};

use crate::core::{Phase, Ctx, Element};

use super::{AlignX, AlignY};


#[derive(Debug, Copy, Clone)]
pub struct Icon {
    texture: Texture2D,
    dest_size: Option<Vec2>,
    pivot_norm: Option<Vec2>,
    color: Color,
    align_x: AlignX,
    align_y: AlignY,
    flip_x: bool,
    flip_y: bool,
    region_norm: Option<Rect>,
}

impl Icon {
    pub fn new(texture: Texture2D) -> Self {
        Icon {
            texture,
            dest_size: None,
            pivot_norm: None,
            color: Default::default(),
            align_x: AlignX::Center,
            align_y: AlignY::Center,
            flip_x: false,
            flip_y: false,
            region_norm: None
        }
    }
}

impl<Event> Element<Event> for Icon {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw => {
                let size = self.dest_size.unwrap_or_else(|| Vec2::new(self.texture.width(), self.texture.height()));
                let pos = Vec2::new(
                    match self.align_x {
                        AlignX::Left => ctx.area.x,
                        AlignX::Center => ctx.area.x + (ctx.area.w - size.x) * 0.5,
                        AlignX::Right => ctx.area.x + ctx.area.w - size.x,
                    },
                    match self.align_y {
                        AlignY::Top => ctx.area.y,
                        AlignY::Center => ctx.area.y + (ctx.area.h - size.y) * 0.5,
                        AlignY::Bottom => ctx.area.y + ctx.area.h - size.y,
                    },
                );
                draw_texture_ex(self.texture, pos.x, pos.y, self.color, DrawTextureParams {
                    dest_size: Some(size),
                    source: self.region_norm.map(|it| Rect::new(it.x * size.x, it.y * size.y, it.w * size.x, it.h * size.y)),
                    rotation: 0.0,
                    flip_x: self.flip_y,
                    flip_y: self.flip_y,
                    pivot: self.pivot_norm.map(|it| it * size),
                });
            }
            Phase::CollectEvents { .. } => {}
        }
    }
}
