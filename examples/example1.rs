use macroquad::color::{BLACK, DARKBLUE, ORANGE, PURPLE, YELLOW};
use macroquad::color::BLUE;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::input::MouseButton::Left;
use macroquad::math::vec2;
use macroquad::texture::Texture2D;
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use rust_macroquad_ui::basic_composites::background::background;
use rust_macroquad_ui::basic_composites::icon::{icon, IconSize};
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::no_stretch::no_stretch;
use rust_macroquad_ui::basic_composites::no_stretch::NoStretchMode::Horizontal;
use rust_macroquad_ui::basic_composites::node_factories::{horizontal_node, stretch_around_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical, StretchSide};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_content, single_content, vertical_content, width};
use rust_macroquad_ui::primitives::conditional::conditional;
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover, on_pressed};
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::primitives::text::TextStyle;
use rust_macroquad_ui::UILayer;
use StretchSide::StretchHorizontal;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Event {
    Click(usize),
    Pressed(usize),
    Hover(usize),
}

#[macroquad::main("UI Example 001")]
async fn main() {
    loop {
        if is_key_pressed(Escape) {
            break;
        }
        do_frame();
        next_frame().await;
    }
}

fn do_frame() {
    clear_background(BLACK);
    let mut layer = UILayer::new(1.0, root());
    layer.update();
    for event in layer.get_events() {
        match event {
            Event::Click(item) => {
                println!("clicked {}", item);
            }
            Event::Hover(_) => {}
            Event::Pressed(_) => {}
        }
    }
    layer.draw();
}

fn root() -> Node<Event> {
    let text_1 = TextStyle {
        font_size: 32.0,
        color: WHITE,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ])
    };

    horizontal_node([
        left_panel(text_1),
        stretch_horizontal(),
        vertical_node([
            stretch_vertical(),
            right_bottom_panel(),
        ])
    ])
}

fn right_bottom_panel() -> Node<Event> {
    node().name("right bottom panel")
        .pad(background(WHITE))
        .pad(margin(8.0))
        .set(horizontal_content(
            [RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE, PURPLE]
                .map(|color|
                    node()
                        .set(single_content(icon(
                            Texture2D::from_file_with_format(include_bytes!("cat.png"), None),
                            BLACK,
                            IconSize::Fixed(32.0, 32.0)
                        )))
                        .pad(margin((8.0, 0.0)))
                        .pad(background(color))
                ),
        ))
}

fn left_panel(text_1: TextStyle) -> Node<Event> {
    node().name("Left panel")
        .pad(background(GREEN))
        .pad(no_stretch(Horizontal))
        .set(vertical_content([
            node()
                .set(vertical_content([
                    stretch_around_node([StretchHorizontal], label("The map", text_1.clone())),
                    node()
                        .pad(background(BLUE))
                        .set(width(150.0))
                        .set(height(150.0)),
                ]))
                .pad(margin(16.0)),
            stretch_vertical(),
            node()
                .pad(margin(8.0))
                .set(vertical_content((0..5)
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
                        .set(single_content(
                            label(format!("Item {:?}", i), text_1.clone())
                        ))
                    )
                    .to_vec())
                ),
        ]))
}
