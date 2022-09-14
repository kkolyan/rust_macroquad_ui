use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::clear_background;
use macroquad::window::next_frame;
use rust_macroquad_ui::basic_composites::backgroud::FluentBackground;
use rust_macroquad_ui::basic_composites::margin::FluentMargin;

use rust_macroquad_ui::fluent_primitives::FluentPrimitives;
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
        .horizontal_group(vec![
            left_panel(text_1),
            node("stretch")
                .height(0.0)
                .width_stretch(),
            node("Right block")
                .vertical_group(vec![
                    node("stretch").height_stretch().width(0.0),
                    right_bottom_panel(),
                ]),
        ])
}

fn right_bottom_panel() -> Node<Event> {
    node("margin")
        .horizontal_group(
            [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE].iter()
                .map(|color| node("color icon")
                    .width(32.0)
                    .height(32.0)
                    .wrap_background(*color)
                    .wrap_margin((8.0, 0.0))
                ).collect(),
        )
        .wrap_margin(8.0)
        .wrap_background(WHITE)
}

fn left_panel(text_1: TextStyle) -> Node<Event> {
    node("Left panel")
        .width_no_stretch()
        .vertical_group(vec![
            node("minimap frame")
                .vertical_group(vec![
                    node("title line")
                        .horizontal_group(vec![
                            node("left stretch")
                                .height(0.0)
                                .width_stretch(),
                            node("map title")
                                .text("The map", text_1),
                            node("left stretch")
                                .height(0.0)
                                .width_stretch(),
                        ]),
                    node("minimap")
                        .width(150.0)
                        .height(150.0)
                        .wrap_background(BLUE),
                ])
                .wrap_margin(16.0),
            node("stretch")
                .width(0.0)
                .height_stretch(),
            node("item box")
                .horizontal_group(vec![
                    node("items list")
                        .vertical_group((0..5)
                            .map(|i| node("Item")
                                .text(format!("Item {}", i), text_1)
                            )
                            .collect())
                        .wrap_background(RED),
                ])
                .wrap_margin(8.0),
        ])
        .wrap_background(GREEN)
}
