use std::os::raw::c_int;
use game_engine::*;

use std::sync::mpsc;
use std::thread;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpriteData {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    r: u8,
    g: u8,
    b: u8,
}

fn fetch_sprite_data(sender: mpsc::Sender<SpriteData>) {
    println!("Fetching sprite data");
    // Perform the HTTP request
    if let Ok(response) = get("https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler") {
        if let Ok(sprite_data) = response.json::<SpriteData>() {
            // Send the data back to the main thread
            let _ = sender.send(sprite_data);
        }
    }
}

fn main() {
    let mut spacebar = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    let (sprite_sender, sprite_receiver) = mpsc::channel();
    let (stop_sender, stop_receiver) = mpsc::channel();
    // Spawn the networking thread
    thread::spawn(move || {
        println!("Starting the fetching sprites thread");
        let mut stop = false;
        while !stop {
            fetch_sprite_data(sprite_sender.clone());
            if let Ok(_) = stop_receiver.try_recv() {
                stop = false;
            }
        }
        drop(sprite_sender);
        println!("Closing the fetching sprites thread");
    });

    println!("Starting the game");
    start_window_and_game_loop!("Test Key Presses: Press Spacebar and all arrows sequentially", 800, 600, 16,
        !(spacebar && up && down && left && right),
        {
            on_key_press!(GLFW_KEY_SPACE, { spacebar = true; });
            on_key_press!(GLFW_KEY_UP, { up = true; });
            on_key_press!(GLFW_KEY_DOWN, { down = true; });
            on_key_press!(GLFW_KEY_LEFT, { left = true; });
            on_key_press!(GLFW_KEY_RIGHT, { right = true; });
            // Check if there's new data from the networking thread
            if let Ok(sprite_data) = sprite_receiver.try_recv() {
                // Use the sprite_data to update your sprite
                // For example, you can spawn a new sprite with the received data
                spawn_sprite!(sprite_data.x as f32, sprite_data.y as f32,
                              sprite_data.width as c_int, sprite_data.height as c_int,
                              sprite_data.r as c_int, sprite_data.g as c_int, sprite_data.b as c_int);
            }
        }
    );
    stop_sender.send(true).unwrap();
}
