#![allow(clippy::new_without_default)]

use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width};
use std::cell::RefCell;
use std::mem;
use std::slice::Iter;
use crate::core::{Ctx, Phase};
use crate::primitives::node::Node;

pub mod composite;
pub mod composite_bounded;
pub mod primitives;
pub mod core;
pub mod basic_composites;
pub mod any_box;

pub struct UILayer<Event> {
    events: Vec<Event>,
    scale: f32,
    root: Node<Event>
}

impl<Event: Clone> UILayer<Event> {
    pub fn new(scale: f32, root: Node<Event>) -> UILayer<Event> {
        UILayer { events: Default::default(), scale, root }
    }

    pub fn get_events(&self) -> Iter<'_, Event> {
        self.events.iter()
    }

    pub fn update(&mut self) {
        self.events.clear();
        let events = RefCell::new(mem::take(&mut self.events));
        self.root.do_phase(Ctx::new(
            screen_rect(),
            self.scale,
            Phase::CollectEvents {
                collected: &events
            },
        ));
        self.events = events.take();
    }

    pub fn draw(&self) {
        self.root.do_phase(Ctx::new(screen_rect(), self.scale, Phase::Draw { events: &self.events }));
    }
}

fn screen_rect() -> Rect {
    Rect::new(0.0, 0.0, screen_width(), screen_height())
}
