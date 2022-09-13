use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::primitives::destretch::DimensionMask;
use rust_macroquad_ui::primitives::fluent::PrimitivesFluentFactory;
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
        .group(Layout::Horizontal, vec![
            lef_panel(text_1),
            Node::new("stretch")
                .height(0.0)
                .width_stretch(),
            Node::new("Right block")
                .group(Layout::Vertical, vec![
                    Node::new("stretch").height_stretch().width(0.0),
                    right_bottom_panel(),
                ]),
        ])
}

fn right_bottom_panel() -> Node<Event> {
    Node::new("margin")
        .background_from_color(WHITE)
        .margin(
            MarginOffset::from(8.0),
            Node::new("right bottom panel")
                .group(
                    Layout::Horizontal,
                    [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE].iter()
                        .map(|color| Node::new("color icon")
                            .margin(
                                MarginOffset::from((8.0, 0.0)),
                                Node::new("icon").background_from_color(*color).width(32.0).height(32.0),
                            )).collect(),
                ),
        )
}

fn lef_panel(text_1: TextStyle) -> Node<Event> {
    Node::new("DeStretch").de_stretch(
        DimensionMask::Horizontal,
        Node::new("Left panel")
            .background_from_color(GREEN)
            .group(Layout::Vertical, vec![
                Node::new("minimap frame")
                    .margin(
                        MarginOffset::from(16.0),
                        Node::new("minimap sub-frame")
                            .group(Layout::Vertical, vec![
                                Node::new("title line")
                                    .group(Layout::Horizontal, vec![
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
                                    .background_from_color(BLUE),
                            ]),
                    ),
                Node::new("stretch")
                    .width(0.0)
                    .height_stretch(),
                Node::new("item box")
                    .margin(
                        MarginOffset::from(8.0),
                        Node::new("items panel").group(Layout::Horizontal, vec![
                            Node::new("items list")
                                .background_from_color(RED)
                                .group(Layout::Vertical, (0..5)
                                    .map(|i| Node::new("Item")
                                        .text(format!("Item {}", i), text_1)
                                    )
                                    .collect()),
                        ]),
                    ),
            ]),
    )
}
