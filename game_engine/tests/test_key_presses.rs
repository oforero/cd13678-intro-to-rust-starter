extern crate game_engine;

use game_engine::*;

fn main() {
    let mut spacebar = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut sprite = create_sprite(0.0, 0.0, 100, 100, 50, 50, 50);

    start_window_and_game_loop!("Test Key Presses: Press Spacebar and all arrows sequentially", 800, 600, 16,
        !(spacebar && up && down && left && right),
        {
            render_sprite(&mut sprite);

            on_key_press!(GLFW_KEY_SPACE, { spacebar = true; });
            on_key_press!(GLFW_KEY_UP, { up = true; });
            on_key_press!(GLFW_KEY_DOWN, { down = true; });
            on_key_press!(GLFW_KEY_LEFT, { left = true; });
            on_key_press!(GLFW_KEY_RIGHT, { right = true; });
        }
    );

}