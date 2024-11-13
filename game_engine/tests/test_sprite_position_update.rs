extern crate game_engine;

use game_engine::*;

fn main() {
    create_game_window("C Test Game", 800, 600);
    let mut x = 0.0;
    let mut y = 0.0;
    let mut sprite = create_sprite(x, y, 50, 100, 100, 50,  50);
    while !window_should_close() {
        update_game_window();
        clear_screen();
        render_sprite(&mut sprite);

        x = if x < 800.0  {
            x + 10.0
        } else {
            0.0
        };
        y = if y < 600.0  {
            y + 10.0
        } else {
            0.0
        };
        update_sprite_position(&mut sprite, x, y);
        std::thread::sleep(std::time::Duration::from_millis(200)); // Optional: Sleep to reduce CPU usage
    }
}