
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
    pub fn create_game_window(
        title: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );

    pub fn create_sprite(
        x: f32,
        y: f32,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        r: ::std::os::raw::c_int,
        g: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    ) -> *mut Sprite;

    pub fn render_sprite(sprite: *mut Sprite);

    pub fn update_sprite_position(sprite: *mut Sprite, x: f32, y: f32);

    pub fn update_game_window();

    pub fn clear_screen();

    pub fn window_should_close() -> ::std::os::raw::c_int;

    pub fn get_key(window: *mut GLFWwindow, key: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    pub fn get_window() -> *mut GLFWwindow;
}