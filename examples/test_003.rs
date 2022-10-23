use std::process::exit;
use macroquad::color::{BLUE, GRAY, WHITE};
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::text::measure_text;
use macroquad::window::next_frame;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::primitives::{horizontal_content, layers, width};
use rust_macroquad_ui::primitives::node::node;
use rust_macroquad_ui::{UILayer};
use rust_macroquad_ui::basic_composites::node_factories::vertical_node;
use rust_macroquad_ui::basic_composites::stretch::stretch_vertical;
use rust_macroquad_ui::primitives::border::border;

#[macroquad::main("test 002")]
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
    let root = vertical_node([
        node::<Event>()
            .name("root")
            .set(border(1.0, WHITE))
            .set(horizontal_content([
                label("Left Click", (30.0, WHITE)),
                label("Num 1", (30.0, WHITE)),
                label("Num g", (30.0, WHITE)),
            ])),
        stretch_vertical(),
    ]);

    let mut layer = UILayer::new(1.0, root);
    layer.update();
    layer.draw();

    // let a = measure_text("Left Click", None, 30, 1.0);
    // let b = measure_text("Num 1", None, 30, 1.0);
    // println!("{}, {}", a.height, b.height);
    //
    // exit(0);
}