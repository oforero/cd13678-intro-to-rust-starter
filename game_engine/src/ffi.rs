use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

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

    pub fn init_gl_system();

    pub fn create_game_window(
        title: *const c_char,
        width: c_int,
        height: c_int,
    );

    pub fn update_game_window();

    pub fn window_should_close() -> c_int;

    pub fn create_sprite(
        x: f32,
        y: f32,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    pub fn render_sprite(sprite: *mut Sprite);

    pub fn update_sprite_position(sprite: *mut Sprite, x: f32, y: f32);


    pub fn clear_screen();

    pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int;

    pub fn get_window() -> *mut GLFWwindow;
}

