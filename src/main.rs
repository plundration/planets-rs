use raylib::prelude::*;

mod body;
use body::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut bodies =
        deserialise_planets(String::from("/home/plunder/coding/planets-rs/scene.json"));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
