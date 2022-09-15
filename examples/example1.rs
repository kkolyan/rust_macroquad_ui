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
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::primitives::{color_fill, height, height_stretch, horizontal_group, vertical_group, width, width_no_stretch, width_stretch};
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

    node("root")
        .set(horizontal_group(vec![
            left_panel(text_1),
            node("stretch")
                .set(height(0.0))
                .set(width_stretch()),
            node("Right block")
                .set(vertical_group(vec![
                    node("stretch").set(height_stretch()).set(width(0.0)),
                    right_bottom_panel(),
                ])),
        ]))
}

fn right_bottom_panel() -> Node<Event> {
    node("margin")
        .set(color_fill(WHITE))
        .set(margin(8.0, node("icons")
            .set(horizontal_group(
                [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE].iter()
                    .map(|color| node("icon")
                        .set(margin((8.0, 0.0), node("color icon")
                            .set(color_fill(*color))
                            .set(width(32.0))
                            .set(height(32.0))))
                    ).collect(),
            ))))
}

fn left_panel(text_1: TextStyle) -> Node<Event> {
    node("Left panel")
        .set(width_no_stretch())
        .set(color_fill(GREEN))
        .set(vertical_group(vec![
            node("minimap frame")
                .set(margin(16.0, node("minimap content")
                    .set(vertical_group(vec![
                        label("The map", (text_1, AlignX::Center, AlignY::Center)),
                        node("minimap")
                            .set(color_fill(BLUE))
                            .set(width(150.0))
                            .set(height(150.0)),
                    ])))),
            node("stretch")
                .set(width(0.0))
                .set(height_stretch()),
            node("item box")
                .set(margin(8.0, node("item box")
                    .set(horizontal_group(vec![
                        node("items list")
                            .set(color_fill(RED))
                            .set(vertical_group((0..5)
                                .map(|i| label(format!("Item {}", i), text_1)
                                )
                                .collect())
                            ),
                    ])))),
        ]))
}
