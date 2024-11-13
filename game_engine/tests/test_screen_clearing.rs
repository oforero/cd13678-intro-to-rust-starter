extern crate game_engine;

use std::time::{Duration, Instant};
use game_engine::*;

fn main() {
    let mut sprite1 = create_sprite(0.0, 0.0, 100, 100, 150, 150, 150);
    let mut sprite2 = create_sprite(5.0, 5.0, 100, 100, 100, 50, 150);

    let mut state = 0; // State to track the current phase
    let mut start_time = Instant::now(); // Track the start time

    start_window_and_game_loop!("Test Screen Clearing", 800, 600, 500, {
       match state {
            0 => {
                // Render the first sprite
                println!("State: {:?}", state);
                clear_screen();
                render_sprite(&mut sprite1);
                if start_time.elapsed() >= Duration::new(2, 0) {
                    state = 1;
                    start_time = Instant::now();
                }
                state = 1;
            }
            1 => {
                // Render the second sprite
                println!("State: {:?}", state);
                clear_screen();
                render_sprite(&mut sprite2);
                if start_time.elapsed() >= Duration::new(2, 0) {
                    state = 0;
                    start_time = Instant::now();
                }
                state = 0;
            }
            _ => {}
       }
    });

}