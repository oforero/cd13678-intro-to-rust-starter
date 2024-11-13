extern crate game_engine;

use game_engine::*;

fn main() {
    let mut spacebar = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut sprite = create_sprite(0.0, 0.0, 100, 100, 50, 50, 50);
    start_window_and_game_loop!("Test Key Presses", 800, 600, 16,
        !(spacebar && up && down && left && right),
        {
            render_sprite(&mut sprite);
            let window_ref = get_window();

            spacebar = spacebar || get_key(window_ref, GLFW_KEY_SPACE) == GLFW_PRESS;
            up = up || get_key(window_ref, GLFW_KEY_UP) == GLFW_PRESS;
            down = down || get_key(window_ref, GLFW_KEY_DOWN) == GLFW_PRESS;
            left = left || get_key(window_ref, GLFW_KEY_LEFT) == GLFW_PRESS;
            right = right || get_key(window_ref, GLFW_KEY_RIGHT) == GLFW_PRESS;
        }
    );

}