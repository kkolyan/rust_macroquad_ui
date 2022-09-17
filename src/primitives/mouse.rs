use std::fmt::Debug;

use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::Vec2;

use crate::core::{Ctx, Element, Phase};

#[derive(Debug, Clone)]
pub struct MouseButtonHandler<Event: Debug + Clone> {
    on_click: Vec<(MouseButton, Event)>,
}

#[derive(Debug, Clone)]
pub struct MouseButtonPressedHandler<Event: Debug + Clone> {
    on_pressed: Vec<(MouseButton, Event)>,
}

#[derive(Debug, Clone)]
pub struct MouseHoverHandler<Event: Debug + Clone> {
    on_hover: Event,
}

impl<Event: Debug + Clone> Element<Event> for MouseButtonHandler<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let area = ctx.area;
        match ctx.phase {
            Phase::Draw { .. } => {}
            Phase::CollectEvents { collected } => {
                let mut hits = None;
                for (button, event_id) in self.on_click.clone() {
                    if hits.is_none() {
                        hits = Some(area.contains(Vec2::from(mouse_position())));
                    }
                    if hits.unwrap() && is_mouse_button_pressed(button) {
                        collected.borrow_mut().push(event_id);
                    }
                }
            }
        }
    }
}

impl<Event: Debug + Clone> Element<Event> for MouseButtonPressedHandler<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        let area = ctx.area;
        match ctx.phase {
            Phase::Draw { .. } => {}
            Phase::CollectEvents { collected } => {
                let mut hits = None;
                for (button, event_id) in self.on_pressed.clone() {
                    if hits.is_none() {
                        hits = Some(area.contains(Vec2::from(mouse_position())));
                    }
                    if hits.unwrap() && is_mouse_button_down(button) {
                        collected.borrow_mut().push(event_id);
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
            Phase::Draw { .. } => {}
            Phase::CollectEvents { collected } => {
                let mut hits = None;
                if hits.is_none() {
                    hits = Some(area.contains(Vec2::from(mouse_position())));
                }
                if hits.unwrap() {
                    collected.borrow_mut().push(self.on_hover.clone());
                }
            }
        }
    }
}

pub fn on_click<Event: Clone + Debug + 'static>(button: MouseButton, event: Event) -> MouseButtonHandler<Event> {
    MouseButtonHandler {
        on_click: vec![(button, event)],
    }
}

pub fn on_pressed<Event: Clone + Debug + 'static>(button: MouseButton, event: Event) -> MouseButtonPressedHandler<Event> {
    MouseButtonPressedHandler {
        on_pressed: vec![(button, event)],
    }
}

pub fn on_hover<Event: Clone + Debug + 'static>(event: Event) -> MouseHoverHandler<Event> {
    MouseHoverHandler {
        on_hover: event,
    }
}
