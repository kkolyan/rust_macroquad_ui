use std::collections::HashSet;
use std::marker::PhantomData;
use macroquad::input::mouse_position;
use macroquad::math::Vec2;
use crate::core::{Ctx, Phase, Element, Flag};
use crate::elements::node::Node;

pub struct StyleHoverTrigger<Event> {
    target: Node<Event>,
    id: Flag,
}

pub struct SelectByStyle<Event, Target: Element<Event>> {
    pd: PhantomData<Event>,
    default: Option<Target>,
    named: Vec<(Flag, Option<Target>)>,
}

impl<Event: Clone> Element<Event> for StyleHoverTrigger<Event> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw => {
                let hits = ctx.area.contains(Vec2::from(mouse_position()));
                if hits {
                    self.target.do_phase(ctx.clone_with(|ctx| assert!(!ctx.flags.insert(self.id), "duplicate flag: {:?}", self.id)));
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

impl<Event, Target: Element<Event>> Element<Event> for SelectByStyle<Event, Target> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        for (name, target) in &self.named {
            if ctx.flags.contains(name) {
                if let Some(target) = target {
                    target.do_phase(ctx);
                }
                return;
            }
        }
        if let Some(target) = &self.default {
            target.do_phase(ctx);
        }
    }
}