//! Module containing the extern C declarations for the open GL wrapper library over the GLFW
//! library.
//!
//! This module is meant to be used internally by the Rust wrappers in the api module.
//!

use std::os::raw::{c_char, c_int};

/// Struct representing a GLFW window
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

/// The rust representation for a GLFW Sprite
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3usize],
    pub x: f32,
    pub y: f32,
}

extern "C" {

    /// Create the GLFW window,
    /// The C wrapper calls the GLFW init function, which means this function can only be called
    /// from the main thread.
    /// The window pointer is not returned but stored in variable in the C library, to get a pointer
    /// to the window use the get_window() function.
    pub fn create_game_window(
        title: *const c_char,
        width: c_int,
        height: c_int,
    );

    /// This function swaps the GLFW buffers and pols the events
    pub fn update_game_window();

    /// Returns 0 if the window has not been closed, 1 otherwise
    pub fn window_should_close() -> c_int;

    /// Creates a rectangular are in the screen, with the given size and color
    /// Returns a pointer to the created sprite
    pub fn create_sprite(
        x: f32,
        y: f32,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    /// Draws the sprite to the background buffer
    pub fn render_sprite(sprite: *mut Sprite);

    /// Update the X and Y positions of a Sprite
    pub fn update_sprite_position(sprite: *mut Sprite, x: f32, y: f32);


    /// Clear the background window buffer
    pub fn clear_screen();

    /// Get the state of a given key: GLFW_PRESS or GLFW_REPEAT
    pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int;

    /// Return a pointer to the GLFW window
    pub fn get_window() -> *mut GLFWwindow;
}

