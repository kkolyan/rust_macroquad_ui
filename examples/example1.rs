use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::fluent_primitives::FluentPrimitives;
use rust_macroquad_ui::primitives::group::Layout;
use rust_macroquad_ui::primitives::margin::MarginOffset;
use rust_macroquad_ui::primitives::node::Node;
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

    Node::<Event>::new("root")
        .horizontal_group(vec![
            left_panel(text_1),
            Node::new("stretch")
                .height(0.0)
                .width_stretch(),
            Node::new("Right block")
                .vertical_group(vec![
                    Node::new("stretch").height_stretch().width(0.0),
                    right_bottom_panel(),
                ]),
        ])
}

fn right_bottom_panel() -> Node<Event> {
    Node::new("margin")
        .color_fill(WHITE)
        .margin(
            MarginOffset::from(8.0),
            Node::new("right bottom panel")
                .horizontal_group(
                    [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE].iter()
                        .map(|color| Node::new("color icon")
                            .margin(
                                MarginOffset::from((8.0, 0.0)),
                                Node::new("icon").color_fill(*color).width(32.0).height(32.0),
                            )).collect(),
                ),
        )
}

fn left_panel(text_1: TextStyle) -> Node<Event> {
    Node::new("Left panel")
        .width_no_stretch()
        .color_fill(GREEN)
        .vertical_group(vec![
            Node::new("minimap frame")
                .margin(
                    MarginOffset::from(16.0),
                    Node::new("minimap sub-frame")
                        .vertical_group(vec![
                            Node::new("title line")
                                .horizontal_group(vec![
                                    Node::new("left stretch")
                                        .height(0.0)
                                        .width_stretch(),
                                    Node::new("map title")
                                        .text("The map", text_1),
                                    Node::new("left stretch")
                                        .height(0.0)
                                        .width_stretch(),
                                ]),
                            Node::new("minimap")
                                .width(150.0)
                                .height(150.0)
                                .color_fill(BLUE),
                        ]),
                ),
            Node::new("stretch")
                .width(0.0)
                .height_stretch(),
            Node::new("item box")
                .margin(
                    MarginOffset::from(8.0),
                    Node::new("items panel").horizontal_group(vec![
                        Node::new("items list")
                            .color_fill(RED)
                            .vertical_group((0..5)
                                .map(|i| Node::new("Item")
                                    .text(format!("Item {}", i), text_1)
                                )
                                .collect()),
                    ]),
                ),
        ])
}
