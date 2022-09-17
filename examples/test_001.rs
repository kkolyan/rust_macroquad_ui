use macroquad::color::{BLUE, GRAY, WHITE};
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::next_frame;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::core::{collect_layer_events, draw_layer};
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_content, vertical_content, width};
use rust_macroquad_ui::primitives::border::border;
use rust_macroquad_ui::primitives::node::node;

#[macroquad::main("test 001")]
async fn main() {
    loop {
        if is_key_pressed(Escape) {
            break;
        }
        do_frame();
        next_frame().await;
    }
}

#[derive(Clone, Debug)]
enum Event {}

fn do_frame() {
    let root = node::<Event>()
        .name("root")
        .set(vertical_content(vec![
            stretch_vertical().name("top area"),
            node()
                .name("label box")
                .set(vertical_content(vec![
                    label("Hi", (30.0, WHITE))
                        .set(border(1.0, GRAY)),
                    stretch_vertical(),
                ])),
        ]));
    let events = collect_layer_events(&root);
    draw_layer(&root, &events);
}