use std::fmt::Debug;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::math::Vec2;
use crate::core::{Ctx, Element, Flag, UiPathStep, Phase};
use crate::primitives::node2::Node;

#[derive(Debug, Clone)]
pub struct MouseHandler<Event: Debug + Clone> {
    on_click: Option<Vec<(MouseButton, Event)>>,
    on_hover: Option<Event>,
}

impl<Event: Debug + Clone> Element<Event> for MouseHandler<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let area = ctx.area;
        match ctx.phase {
            Phase::Draw => {}
            Phase::CollectEvents { on_event } => {
                let mut hits = None;
                if let Some(on_click) = &self.on_click {
                    for (button, event_id) in on_click {
                        if hits.is_none() {
                            hits = Some(area.contains(Vec2::from(mouse_position())));
                        }
                        if hits.unwrap() && is_mouse_button_down(*button) {
                            on_event(event_id);
                        }
                    }
                }
                if let Some(on_hover) = &self.on_hover {
                    if hits.is_none() {
                        hits = Some(area.contains(Vec2::from(mouse_position())));
                    }
                    if hits.unwrap() {
                        on_event(on_hover);
                    }
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
                    self.target.do_phase(ctx);
                };
            }
            Phase::CollectEvents { .. } => {
                self.target.do_phase(ctx);
            }
        }
    }
}
