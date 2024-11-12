use std::ffi::CString;
use std::os::raw::c_int;

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub color: [::std::os::raw::c_int; 3usize],
    pub x: f32,
    pub y: f32,
}

extern "C" {

    fn init_gl_system();

    fn create_game_window(
        title: *const ::std::os::raw::c_char,
        width: c_int,
        height: c_int,
    );

    fn update_game_window();

    fn window_should_close() -> c_int;

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

pub fn rust_create_game_window(
    title: &str,
    width: c_int,
    height: c_int) {
    let c_string = CString::new(title).expect("CString::new failed");
    unsafe {
        create_game_window(c_string.as_ptr(), width, height);
    }
}

pub fn rust_init_gl_system() {
    unsafe {
        init_gl_system();
    }
}

pub fn rust_update_game_window() {
    unsafe {
        update_game_window();
    }
}

pub fn rust_window_should_close() -> bool {
    unsafe { window_should_close() != 0 }
}
