use macroquad::color::BLACK;
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::elements::background::BackgroundFactory;
use rust_macroquad_ui::elements::common::AlignX;
use rust_macroquad_ui::elements::common::AlignY;
use rust_macroquad_ui::elements::group::GroupFactory;
use rust_macroquad_ui::elements::group::HeightFactory;
use rust_macroquad_ui::elements::group::Layout;
use rust_macroquad_ui::elements::group::WidthFactory;
use rust_macroquad_ui::elements::margin::MarginFactory;
use rust_macroquad_ui::elements::margin::MarginOffset;
use rust_macroquad_ui::elements::node::Node;
use rust_macroquad_ui::elements::text::TextFactory;

#[derive(Clone, Debug)]
enum Event {}

#[macroquad::main("UI Example 001")]
async fn main() {
    let root = Node::<Event>::new("root")
        .group(Layout::Horizontal, vec![
            Node::new("Left block")
                .background_from_color(GREEN)
                .group(Layout::Vertical, vec![
                    Node::new("minimap frame")
                        .margin(
                            MarginOffset::from(16.0),
                            Node::new("minimap")
                                .width(150.0)
                                .height(150.0)
                                .background_from_color(BLUE)
                        ),
                    Node::new("stretch")
                        .width(0.0)
                        .height_stretch(),
                    Node::new("items panel").group(Layout::Horizontal, vec![
                        Node::new("items list")
                            .background_from_color(RED)
                            .group(Layout::Vertical, (0..4)
                                .map(|i| Node::new("Item")
                                    .width(150.0)
                                    .text(format!("Item {}", i), 32.0, WHITE, AlignX::Left, AlignY::Center)
                                    .height(32.0)
                                )
                                .collect()),
                    ])
                ]),
            Node::new("action area")
                .width_stretch(),
        ]);
    loop {
        clear_background(BLACK);
        let _ = rust_macroquad_ui::core::collect_layer_events(&root);
        rust_macroquad_ui::core::draw_layer(&root);
        next_frame().await;
    }
}
