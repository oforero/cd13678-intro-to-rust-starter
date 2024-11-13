extern crate game_engine;

use game_engine::*;

fn main() {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut sprite = create_sprite(x, y, 50, 100, 100, 50,  50);

    start_window_and_game_loop!("Test Sprite Position Update", 800, 600, 16, {
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
    });

}