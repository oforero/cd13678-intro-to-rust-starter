extern crate game_engine;

use game_engine::*;

fn main() {
    start_window_and_game_loop!("Test Sprite Rendering", 800, 600, 16, {
        spawn_sprite!(0.0, 0.0, 100, 100, 50, 50, 50);
    });
}