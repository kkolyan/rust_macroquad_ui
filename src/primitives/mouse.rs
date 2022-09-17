use std::fmt::Debug;
use macroquad::input::{ is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::Vec2;
use crate::core::{Ctx, Element, Flag, UiPathStep, Phase};
use crate::primitives::node::Node;

#[derive(Debug, Clone)]
pub struct MouseButtonHandler<Event: Debug + Clone> {
    on_click: Vec<(MouseButton, Event)>,
}

#[derive(Debug, Clone)]
pub struct MouseHoverHandler<Event: Debug + Clone> {
    on_hover: Event,
}

impl<Event: Debug + Clone> Element<Event> for MouseButtonHandler<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let area = ctx.area;
        match ctx.phase {
            Phase::Draw => {}
            Phase::CollectEvents { on_event } => {
                let mut hits = None;
                for (button, event_id) in &self.on_click {
                    if hits.is_none() {
                        hits = Some(area.contains(Vec2::from(mouse_position())));
                    }
                    if hits.unwrap() && is_mouse_button_pressed(*button) {
                        on_event(event_id);
                    }
                }
            }
        }
    }
}

impl<Event: Debug + Clone> Element<Event> for MouseHoverHandler<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let area = ctx.area;
        match ctx.phase {
            Phase::Draw => {}
            Phase::CollectEvents { on_event } => {
                let mut hits = None;
                if hits.is_none() {
                    hits = Some(area.contains(Vec2::from(mouse_position())));
                }
                if hits.unwrap() {
                    on_event(&self.on_hover);
                }
            }
        }
    }
}

pub struct FlagOnHover<Event> {
    target: Node<Event>,
    flag: Flag,
}

impl<Event: Clone> Element<Event> for FlagOnHover<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let ctx = ctx.step_down(UiPathStep::Name("FlagOnHover"));
        match ctx.phase {
            Phase::Draw => {
                let hits = ctx.area.contains(Vec2::from(mouse_position()));
                if hits {
                    self.target.do_phase(ctx.clone_with(|ctx| assert!(!ctx.flags.insert(self.flag), "duplicate flag: {:?}", self.flag)));
                } else {
                    self.target.do_phase(ctx.clone());
                };
            }
            Phase::CollectEvents { .. } => {
                self.target.do_phase(ctx.clone());
            }
        }
    }
}

pub fn on_click<Event: Clone + Debug + 'static>(button: MouseButton, event: Event) -> MouseButtonHandler<Event> {
    MouseButtonHandler {
        on_click: vec![(button, event)],
    }
}

pub fn on_hover<Event: Clone + Debug + 'static>(event: Event) -> MouseHoverHandler<Event> {
    MouseHoverHandler {
        on_hover: event,
    }
}
