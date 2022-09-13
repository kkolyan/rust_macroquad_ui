use std::marker::PhantomData;
use crate::core::{Ctx, Element, Flag};

pub struct Conditional<Event, Target: Element<Event>> {
    pd: PhantomData<Event>,
    default: Option<Target>,
    named: Vec<(Flag, Option<Target>)>,
}

impl<Event, Target: Element<Event>> Element<Event> for Conditional<Event, Target> {
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