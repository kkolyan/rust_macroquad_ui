use macroquad::color::{BLUE, GREEN, RED, WHITE};
use macroquad::window::next_frame;
use rust_macroquad_ui::elements::{AlignX, AlignY};
use rust_macroquad_ui::elements::background::{Background, BackgroundFactory};
use rust_macroquad_ui::elements::group::{Group, GroupFactory, HeightFactory, Layout, Width, WidthFactory};
use rust_macroquad_ui::elements::name::NameFactory;
use rust_macroquad_ui::elements::node::Node;
use rust_macroquad_ui::elements::text::TextFactory;

#[derive(Clone, Debug)]
enum Event {}

#[macroquad::main("UI Example 001")]
async fn main() {
    loop {
        let root = Node::<Event>::new()
            .group(Layout::Horizontal, vec![
                Node::new()
                    .name("Screen")
                    .width(300.0)
                    .background_from_color(GREEN)
                    .group(Layout::Vertical, vec![
                        Node::new().name("map border top").height(16.0),
                        Node::new()
                            .name("map column")
                            .height(200.0)
                            .group(Layout::Horizontal, vec![
                                Node::new().name("map border left").width(16.0),
                                Node::new()
                                    .name("map")
                                    .width(200.0)
                                    .background_from_color(BLUE),
                                Node::new().name("map border right").width(16.0),
                            ]),
                        Node::new().name("map border bottom").height(16.0),
                        Node::new().name("items panel").height(200.0).group(Layout::Horizontal, vec![
                            Node::new()
                                .name("items list")
                                .width(150.0)
                                .background_from_color(RED)
                                .group(Layout::Vertical, (0..10)
                                    .map(|i| Node::new()
                                        .text(format!("Item {}", i), 32.0, WHITE, AlignX::Left, AlignY::Center)
                                        .height(32.0)
                                    )
                                    .collect()),
                        ])
                    ])
            ]);
        let events = rust_macroquad_ui::core::collect_layer_events(&root);
        rust_macroquad_ui::core::draw_layer(&root);
        next_frame().await;
    }
}
