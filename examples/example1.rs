use std::collections::HashMap;
use macroquad::color::{BLACK, BLUE, GREEN, RED, WHITE};
use macroquad::window::{clear_background, next_frame};
use rust_macroquad_ui::elements::background::{Background, BackgroundFactory};
use rust_macroquad_ui::elements::common::{AlignX, AlignY};
use rust_macroquad_ui::elements::group::{Group, GroupFactory, HeightFactory, Layout, Width, WidthFactory};
use rust_macroquad_ui::elements::margin::{MarginFactory, MarginOffset};
use rust_macroquad_ui::elements::name::NameFactory;
use rust_macroquad_ui::elements::node::Node;
use rust_macroquad_ui::elements::text::TextFactory;

#[derive(Clone, Debug)]
enum Event {}

#[macroquad::main("UI Example 001")]
async fn main() {
    let root = Node::<Event>::new()
        .group(Layout::Horizontal, vec![
            Node::new()
                .name("Left block")
                .background_from_color(GREEN)
                .group(Layout::Vertical, vec![
                    Node::new()
                        .name("minimap frame")
                        .margin(
                            MarginOffset::from(16.0),
                            Node::new()
                                .name("minimap")
                                .width(150.0)
                                .height(150.0)
                                .background_from_color(BLUE)
                        ),
                    Node::new().name("stretch")
                        .width(0.0)
                        .height_stretch(),
                    Node::new().name("items panel").group(Layout::Horizontal, vec![
                        Node::new()
                            .name("items list")
                            .background_from_color(RED)
                            .group(Layout::Vertical, (0..4)
                                .map(|i| Node::new()
                                    .name("Item")
                                    .width(150.0)
                                    .text(format!("Item {}", i), 32.0, WHITE, AlignX::Left, AlignY::Center)
                                    .height(32.0)
                                )
                                .collect()),
                    ])
                ]),
            Node::new()
                .name("action area")
                .width_stretch(),
        ]);
    loop {
        clear_background(BLACK);
        let events = rust_macroquad_ui::core::collect_layer_events(&root);
        rust_macroquad_ui::core::draw_layer(&root);
        next_frame().await;
    }
}
