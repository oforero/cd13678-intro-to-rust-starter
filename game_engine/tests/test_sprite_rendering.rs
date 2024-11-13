extern crate game_engine;

use game_engine::*;

fn main() {
    create_game_window("C Test Game", 800, 600);
    while !window_should_close() {
        update_game_window();
        let mut sprite = create_sprite(0.0, 0.0, 100, 100, 50, 50, 50);
        render_sprite(&mut sprite);
        std::thread::sleep(std::time::Duration::from_millis(16)); // Optional: Sleep to reduce CPU usage
    }
}