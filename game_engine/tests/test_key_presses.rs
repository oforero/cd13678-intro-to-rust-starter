extern crate game_engine;

use game_engine::*;

fn main() {
    create_game_window("C Test Game", 800, 600);
    let mut spacebar = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    while !(spacebar && up && down && left && right) && !window_should_close() {
        update_game_window();
        let mut sprite = create_sprite(0.0, 0.0, 100, 100, 50, 50, 50);
        render_sprite(&mut sprite);
        let window_ref = get_window();

        spacebar = spacebar || get_key(window_ref, GLFW_KEY_SPACE) == GLFW_PRESS;
        up = up || get_key(window_ref, GLFW_KEY_UP) == GLFW_PRESS;
        down = down || get_key(window_ref, GLFW_KEY_DOWN) == GLFW_PRESS;
        left = left || get_key(window_ref, GLFW_KEY_LEFT) == GLFW_PRESS;
        right = right || get_key(window_ref, GLFW_KEY_RIGHT) == GLFW_PRESS;

        std::thread::sleep(std::time::Duration::from_millis(16)); // Optional: Sleep to reduce CPU usage
    }
}