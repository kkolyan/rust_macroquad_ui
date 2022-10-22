use macroquad::color::{BLUE, GRAY, WHITE};
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::next_frame;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::primitives::{layers, width};
use rust_macroquad_ui::primitives::node::node;
use rust_macroquad_ui::{UILayer};

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
    let root = node::<Event>()
        .name("root")
        .set(layers([
            label("test", (30.0, WHITE)),
            label("_dsa_", (30.0, WHITE)),
        ]));

    let mut layer = UILayer::new(1.0, root);
    layer.update();
    layer.draw();
}