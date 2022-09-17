use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use macroquad::math::Rect;
use macroquad::window::screen_height;
use macroquad::window::screen_width;

use crate::primitives::node::Node;


#[derive(Clone)]
pub enum Phase<'a, Event> {
    Draw { events: &'a Vec<Event> },
    CollectEvents { collected: Rc<RefCell<Vec<Event>>> },
}

#[derive(Clone)]
pub struct Ctx<'a, Event> {
    pub area: Rect,
    pub scale: f32,
    pub phase: Phase<'a, Event>,
    pub path: VecDeque<UiPathStep>,
}

#[derive(Copy, Clone, Debug)]
pub enum UiPathStep {
    Name(&'static str),
    Index(usize),
}

pub trait Element<Event> {
    fn do_phase(&self, _ctx: Ctx<Event>) {}
    fn expand_padding(self) -> Self
        where Self: Sized {
        self
    }
}

pub fn collect_layer_events<Event: 'static + Clone>(layer_root: &Node<Event>) -> Vec<Event> {
    let events = Rc::new(RefCell::from(vec![]));
    layer_root.do_phase(Ctx::new(
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        1.0,
        Phase::CollectEvents {
            collected: events.clone()
        },
    ));
    events.take()
}

pub fn draw_layer<Event: Clone>(layer_root: &Node<Event>, events: &Vec<Event>) {
    layer_root.do_phase(Ctx::new(Rect::new(0.0, 0.0, screen_width(), screen_height()), 1.0, Phase::Draw { events }));
}

impl<'a, Event: Clone> Ctx<'a, Event> {
    pub fn new(area: Rect, scale: f32, phase: Phase<'a, Event>) -> Self {
        Ctx {
            area,
            scale,
            phase,
            path: Default::default(),
        }
    }

    pub fn backtrace(&self) -> String {
        let mut s = String::new();
        for step in &self.path {
            s.push('/');
            match step {
                UiPathStep::Name(name) => s.push_str(name),
                UiPathStep::Index(index) => s.push_str(format!("{}", index).as_str())
            }
        }
        s
    }

    pub fn clone_with<F: Fn(&mut Self)>(&self, f: F) -> Self {
        let mut v: Self = self.clone();
        f(&mut v);
        v
    }

    pub fn step_down(&self, step: UiPathStep) -> Self {
        let mut v: Self = self.clone();
        v.path.push_back(step);
        v
    }
}

// impl <'a, Event> Ctx<'a, Event> {
//     pub fn new(area: Rect, scale: f32, phase: Phase<'a, Event>) -> Ctx<Event> {
//         Ctx { area, scale, flags: HashSet::new(), phase }
//     }
//
//     pub fn area(&self) -> Rect {
//         self.area
//     }
//
//     pub fn scale(&self) -> f32 {
//         self.scale.clone()
//     }
//
//     pub fn phase(&mut self) -> &mut Phase<'a, Event> {
//         &mut self.phase
//     }
//
//     pub fn flags(&self) -> &HashSet<Flag> {
//         &self.flags
//     }
//
//     pub fn with_scale<F: Fn(&mut Ctx<Event>)>(&mut self, scale: f32, f: F) {
//         let prev = self.scale.clone();
//         self.scale = scale;
//         f(self);
//         self.scale = prev;
//     }
//
//     pub fn with_area<F: Fn(&mut Ctx<Event>)>(&mut self, area: Rect, f: F) {
//         let prev = self.area;
//         self.area = area;
//         f(self);
//         self.area = prev;
//     }
//
//     pub fn with_flag<F: FnMut(&mut Ctx<Event>)>(&mut self, flag: Flag, mut f: F) {
//         assert!(!self.flags.insert(flag), "duplicate flag");
//         f(self);
//         self.flags.remove(&flag);
//     }
// }
