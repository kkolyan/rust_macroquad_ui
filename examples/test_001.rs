use macroquad::color::{BLUE, GRAY, WHITE};
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode::Escape;
use macroquad::window::next_frame;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::stretch::{stretch_vertical};
use rust_macroquad_ui::primitives::{ vertical_content, width};
use rust_macroquad_ui::primitives::border::border;
use rust_macroquad_ui::primitives::node::node;
use rust_macroquad_ui::{UILayer};

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
                    label("0 Hiyp", (30.0, WHITE))
                        .set(border(1.0, GRAY)),
                    stretch_vertical(),
                ])),
        ]));

    let mut layer = UILayer::new(1.0, root);
    layer.update();
    layer.draw();
}