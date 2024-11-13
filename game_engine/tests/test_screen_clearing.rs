extern crate game_engine;

use std::time::{Duration, Instant};
use game_engine::*;

fn main() {
    create_game_window("C Test Game", 800, 600);

    let mut sprite1 = create_sprite(0.0, 0.0, 100, 100, 150, 150, 150);
    let mut sprite2 = create_sprite(5.0, 5.0, 100, 100, 100, 50, 150);

    let mut state = 0; // State to track the current phase
    let mut start_time = Instant::now(); // Track the start time

    while !window_should_close() {
        update_game_window();

        match state {
            0 => {
                // Render the first sprite
                println!("State: {:?}", state);
                clear_screen();
                render_sprite(&mut sprite1);
                state = 1;
            }
            1 => {
                println!("State: {:?}", state);
                if start_time.elapsed() >= Duration::new(5, 0) {
                    // After 3 seconds, clear the screen and move to the next state
                    clear_screen();
                    state = 2; // Move to the next state
                    // Reset the timer for the next state
                    start_time = Instant::now();
                }
            }
            2 => {
                // Wait for 1 second
                println!("State: {:?}", state);
                if start_time.elapsed() >= Duration::new(2, 0) {
                    // After 1 second, render the second sprite
                    clear_screen();
                    render_sprite(&mut sprite2);
                    state = 3; // Move to the next state
                    // Reset the timer for the next state
                    start_time = Instant::now();
                }
            }
            3 => {
                // Optionally, you can keep the second sprite on the screen or loop back
                // For example, you could clear the screen again after some time
                println!("State: {:?}", state);
                if start_time.elapsed() >= Duration::new(5, 0) {
                    clear_screen();
                    state = 0; // Loop back to the first sprite
                    start_time = Instant::now(); // Reset the timer
                }
            }
            _ => {}
        }

        // Sleep to reduce CPU usage
        // std::thread::sleep(Duration::from_millis(16));
    }
}