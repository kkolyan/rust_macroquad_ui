use std::fmt::Debug;
use macroquad::color::Color;
use crate::core::Flag;
use crate::primitives::color_fill::ColorFill;
use crate::primitives::conditional::Conditional;
use crate::primitives::mouse::FlagOnHover;
use crate::primitives::node::{Node, node};

#[derive(Debug, Copy, Clone)]
pub struct Button<Event> {
    pub event: Option<Event>,
    pub background: Option<Color>,
    pub background_hover: Option<Color>,
}

pub trait FluentButton<Event> {
    fn wrap_button<S: Into<Button<Event>>>(self, settings: S) -> Self;
}

impl<Event: Clone + Debug + 'static> FluentButton<Event> for Node<Event> {
    fn wrap_button<S: Into<Button<Event>>>(self, settings: S) -> Self {
        let settings = settings.into();
        node("button")
            .add_component(FlagOnHover {
                target: node("button_clicker")
                    .add_component(Conditional {
                        pd: Default::default(),
                        default: settings.background.map(|it| ColorFill {color: it}),
                        named: vec![
                            (Flag("hover"), settings.background_hover.map(|it| ColorFill {color: it})),
                        ]
                    }),
                flag: Flag("hover"),
            })
    }
}