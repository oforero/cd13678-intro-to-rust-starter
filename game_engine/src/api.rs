use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::null;
use super::ffi;

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

pub use ffi::Sprite;
use crate::ffi::GLFWwindow;

pub fn create_game_window(
    title: &str,
    width: c_int,
    height: c_int) {
    let c_string = CString::new(title).expect("CString::new failed");
    unsafe {
        ffi::create_game_window(c_string.as_ptr(), width, height);
    }
}

pub fn update_game_window() {
    unsafe {
        ffi::update_game_window();
    }
}

pub fn window_should_close() -> bool {
    unsafe { ffi::window_should_close() != 0 }
}

pub fn create_sprite(
    x: f32,
    y: f32,
    width: c_int,
    height: c_int,
    r: c_int,
    g: c_int,
    b: c_int,
) -> Sprite {
    let mut sprite: Sprite;
    unsafe {
        sprite = *ffi::create_sprite(x, y, width, height, r, g, b);
    }
    sprite
}

pub fn render_sprite(sprite: &mut Sprite) {
    unsafe {
        ffi::render_sprite(sprite);
    }
}

pub fn update_sprite_position(sprite: &mut Sprite, x: f32, y: f32) {
    unsafe {
        ffi::update_sprite_position(sprite, x, y);
    }
}


pub fn clear_screen() {
    unsafe {
        ffi::clear_screen();
    }
}

pub fn get_window() -> *mut GLFWwindow {
    unsafe {
        ffi::get_window()
    }
}
pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int {
    unsafe {
        ffi::get_key(window, key)
    }
}