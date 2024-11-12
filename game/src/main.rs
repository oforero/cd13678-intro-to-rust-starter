use game_engine::*;

fn main() {
    rust_create_game_window("C Test Game", 800, 600);
    while !rust_window_should_close() {
        rust_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16)); // Optional: Sleep to reduce CPU usage
    }
}
