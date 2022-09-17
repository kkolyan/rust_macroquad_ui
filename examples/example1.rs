
use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::input::MouseButton::Left;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::basic_composites::align::{AlignX, AlignY};
use rust_macroquad_ui::basic_composites::background::background;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::no_stretch::no_stretch;
use rust_macroquad_ui::basic_composites::no_stretch::NoStretchMode::Horizontal;
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_group, single, vertical_group, width};
use rust_macroquad_ui::primitives::conditional::{conditional};
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover, on_pressed};
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::primitives::text::TextStyle;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Event {
    Click(usize),
    Pressed(usize),
    Hover(usize),
}

struct App {
}

#[macroquad::main("UI Example 001")]
async fn main() {
    let mut app = App {
    };
    loop {
        if is_key_pressed(Escape) {
            break;
        }
        do_frame(&mut app);
        next_frame().await;
    }
}

fn do_frame(_: &mut App) {
    clear_background(BLACK);
    let events = rust_macroquad_ui::core::collect_layer_events(&root());
    for event in events.iter() {
        match event {
            Event::Click(item) => {
                println!("clicked {}", item);
            }
            Event::Hover(_) => {
            }
            Event::Pressed(_) => {}
        }
    }

    rust_macroquad_ui::core::draw_layer(&root(), &events);
}

fn root() -> Node<Event> {
    let text_1 = TextStyle {
        font_size: 32.0,
        color: WHITE,
    };

    node().name("root")
        .set(horizontal_group(vec![
            left_panel(text_1),
            stretch_horizontal(),
            node().name("Right block")
                .set(vertical_group(vec![
                    stretch_vertical(),
                    right_bottom_panel(),
                ])),
        ]))
}

fn right_bottom_panel() -> Node<Event> {
    node().name("right bottom panel")
        .pad(background(WHITE))
        .pad(margin(8.0))
        .set(horizontal_group(
            [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE].iter()
                .map(|color| node().name("icon")
                    .pad(margin((8.0, 0.0)))
                    .pad(background(color))
                    .set(width(32.0))
                    .set(height(32.0))
                ).collect(),
        ))
}

fn left_panel(text_1: TextStyle) -> Node<Event> {
    node().name("Left panel")
        .pad(background(GREEN))
        .pad(no_stretch(Horizontal))
        .set(vertical_group(vec![
            node()
                .set(vertical_group(vec![
                    label("The map", (text_1, AlignX::Center, AlignY::Center)),
                    node()
                        .pad(background(BLUE))
                        .set(width(150.0))
                        .set(height(150.0)),
                ]))
                .pad(margin(16.0)),
            stretch_vertical(),
            node()
                .pad(margin(8.0))
                .set(vertical_group((0..5)
                    .map(|i| node()
                        .set(on_click(Left, Event::Click(i)))
                        .set(on_hover(Event::Hover(i)))
                        .set(on_pressed(Left, Event::Pressed(i)))
                        .set(conditional((
                            Some(color_fill(RED)),
                            [
                                (Event::Pressed(i), Some(color_fill(YELLOW))),
                                (Event::Hover(i), Some(color_fill(ORANGE))),
                            ]
                        )))
                        .set(single(
                            label(format!("Item {:?}", i), text_1)
                        ))
                    )
                    .collect())
                ),
        ]))
}
