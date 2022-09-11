use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width};
use crate::make_bounded_composite;

make_bounded_composite! {pub, ComponentSet<Event>, Element<Event>}


#[derive(Clone)]
pub enum Phase<Event> {
    Draw,
    CollectEvents { on_event: Rc<dyn Fn(&Event)> },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flag(pub &'static str);

#[derive(Clone)]
pub struct Ctx<Event> {
    pub area: Rect,
    pub scale: f32,
    pub flags: HashSet<Flag>,
    pub phase: Phase<Event>,
}

pub trait Element<Event> {
    fn do_phase(&self, _ctx: Ctx<Event>) {}
}

pub fn collect_layer_events<Event: 'static +  Clone, Root: Element<Event>>(layer_root: &Root) -> Vec<Event> {
    let events = Rc::new(RefCell::from(vec![]));
    let events2 = events.clone();
    layer_root.do_phase(Ctx::new(
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        1.0,
        Phase::CollectEvents {
            on_event: Rc::new(move |it| events2.borrow_mut().push(it.clone()))
        }
    ));
    events.take()
}

pub fn draw_layer<Event: Clone, Root: Element<Event>>(layer_root: &Root) {
    layer_root.do_phase(Ctx::new(Rect::new(0.0, 0.0, screen_width(), screen_height()), 1.0, Phase::Draw));
}

impl <Event: Clone> Ctx<Event> {
    pub fn new(area: Rect, scale: f32, phase: Phase<Event>) -> Self {
        Ctx {
            area,
            scale,
            phase,
            flags: Default::default(),
        }
    }

    pub fn clone_with<F: Fn(&mut Self)>(&self, f: F) -> Self {
        let mut v: Self = self.clone();
        f(&mut v);
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
