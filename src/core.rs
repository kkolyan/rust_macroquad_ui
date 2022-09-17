use std::cell::RefCell;
use std::fmt::Debug;

use macroquad::math::Rect;


#[derive(Clone, Copy, Debug)]
pub enum Phase<'a, Event> {
    Draw { events: &'a Vec<Event> },
    CollectEvents { collected: &'a RefCell<Vec<Event>> },
}

#[derive(Clone, Copy, Debug)]
pub struct Ctx<'a, Event> {
    pub area: Rect,
    pub scale: f32,
    pub phase: Phase<'a, Event>,
    pub path: UiPathStep<'a>,
}

#[derive(Copy, Clone, Debug)]
pub enum UiPathStep<'a> {
    Name(&'static str, Option<&'a UiPathStep<'a>>),
    Index(usize, Option<&'a UiPathStep<'a>>),
}

pub trait Element<Event> {
    fn do_phase(&self, _ctx: Ctx<Event>) {}
    fn expand_padding(self) -> Self
        where Self: Sized {
        self
    }
}

impl<'a, Event: Clone> Ctx<'a, Event> {
    pub fn new(area: Rect, scale: f32, phase: Phase<'a, Event>) -> Self {
        Ctx {
            area,
            scale,
            phase,
            path: UiPathStep::Name("root", None),
        }
    }

    pub fn backtrace(&self) -> String {
        let mut parts = vec![];
        let mut _step = Some(&self.path);
        while let Some(step) = _step {
            _step = match *step {
                UiPathStep::Name(name, parent) => {
                    parts.push(name.to_owned());
                    parent
                }
                UiPathStep::Index(index, parent) => {
                    parts.push(format!("{}", index));
                    parent
                }
            };
        }

        let mut s = String::new();
        for part in parts.iter().rev() {
            s.push('/');
            s.push_str(part);
        }
        s
    }

    pub fn clone_with<F: Fn(&mut Self)>(&self, f: F) -> Self {
        let mut v: Self = self.clone();
        f(&mut v);
        v
    }

    pub fn step_down(&'a self, step: &'static str) -> Self {
        let mut v: Self = self.clone();
        v.path = UiPathStep::Name(step, Some(&self.path));
        v
    }

    pub fn step_down_i(&'a self, step: usize) -> Self {
        let mut v: Self = self.clone();
        v.path = UiPathStep::Index(step, Some(&self.path));
        v
    }
}
