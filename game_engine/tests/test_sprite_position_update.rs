extern crate game_engine;

use game_engine::*;

fn main() {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut sprite = create_sprite(x, y, 50, 100, 100, 50,  50);

    start_window_and_game_loop!("Test Sprite Position Update", 800, 600, 16, {
        clear_screen();
        if x < 800.0  {
            move_sprite!(false, &mut sprite, 10.0, 0.0);
        };

        if y < 600.0  {
            move_sprite!(false, &mut sprite, 0.0, 10.0);
        };

    });

}