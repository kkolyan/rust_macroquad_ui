use std::fmt::Debug;
use std::marker::PhantomData;
use crate::core::{Ctx, Element, Phase};

#[derive(Clone, Debug)]
pub struct EventBased<Event, Target: Element<Event>> {
    pd: PhantomData<Event>,
    default: Option<Target>,
    conditional: Vec<(Event, Option<Target>)>,
}

impl<Event, Target: Element<Event>> EventBased<Event, Target> {
    fn new(default: Option<Target>, conditional: Vec<(Event, Option<Target>)>) -> Self {
        EventBased {
            pd: Default::default(),
            default,
            conditional,
        }
    }
}

impl<Event: Eq + PartialEq, Target: Element<Event>> Element<Event> for EventBased<Event, Target> {
    fn do_phase(&self, ctx: Ctx<Event>) {
        match ctx.phase {
            Phase::Draw { events } => {
                for (event, target) in &self.conditional {
                    if events.contains(event) {
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
            Phase::CollectEvents { .. } => {}
        }
    }
}

impl<Event: Clone, Target: Element<Event> + Clone, const N: usize>
From<(Option<Target>, [(Event, Option<Target>); N])>
for EventBased<Event, Target> {
    fn from((default, conditional): (Option<Target>, [(Event, Option<Target>); N])) -> Self {
        EventBased::new(default, conditional.to_vec())
    }
}

impl<Event, Target: Element<Event>>
From<(Option<Target>, Vec<(Event, Option<Target>)>)>
for EventBased<Event, Target> {
    fn from((default, conditional): (Option<Target>, Vec<(Event, Option<Target>)>)) -> Self {
        EventBased::new(default, conditional)
    }
}

pub fn conditional<Event, Target, T>(v: T) -> EventBased<Event, Target>
    where
        Target: Element<Event>,
        T: Into<EventBased<Event, Target>> {
    v.into()
}
