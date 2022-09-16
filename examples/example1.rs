use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::basic_composites::align::{AlignX, AlignY};
use rust_macroquad_ui::basic_composites::background::background;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::no_stretch::no_stretch;
use rust_macroquad_ui::basic_composites::no_stretch::NoStretchMode::Horizontal;
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_group, vertical_group, width, width_no_stretch};
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::primitives::text::TextStyle;

#[derive(Clone, Debug)]
enum Event {}

#[macroquad::main("UI Example 001")]
async fn main() {
    loop {
        if is_key_pressed(Escape) {
            break;
        }
        clear_background(BLACK);
        let root = root();
        let _ = rust_macroquad_ui::core::collect_layer_events(&root);
        rust_macroquad_ui::core::draw_layer(&root);
        next_frame().await;
    }
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
                    .pad(background(*color))
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
                .pad(background(RED))
                .set(vertical_group((0..5)
                    .map(|i| label(format!("Item {}", i), text_1))
                    .collect())
                ),
        ]))
}
