
/// Generates a standard game loop
///
/// Required `use` statements:
/// ```rust
/// use game_engine;
/// ```
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title: expr, $width: expr, $height: expr, $millis: expr, $code:block) => {
        game_engine::create_game_window($title, $width, $height);
        while !game_engine::window_should_close() {
            game_engine::update_game_window();
            $code
            std::thread::sleep(std::time::Duration::from_millis($millis));
        }
    };
    ($title: expr, $width: expr, $height: expr, $millis: expr, $condition: expr, $code:block) => {
        game_engine::create_game_window($title, $width, $height);
        while $condition && !game_engine::window_should_close() {
            game_engine::update_game_window();
            $code
            std::thread::sleep(std::time::Duration::from_millis($millis));
        }
    };
}

#[macro_export]
macro_rules! on_key_press {
    ($key: ident, $handler: block) => {
        if game_engine::get_key(get_window(), $key) == game_engine::GLFW_PRESS {
            $handler
        }
    };
}

#[macro_export]
macro_rules! spawn_sprite {
    ($x: expr, $y: expr, $w: expr, $h: expr, $r: expr, $g: expr, $b: expr) => {
        let mut sprite = game_engine::create_sprite($x, $y, $w, $h, $r, $g, $b);
        game_engine::render_sprite(&mut sprite);
        sprite
    }
}

#[macro_export]
macro_rules! move_sprite {
    ($clear: expr, $sprite: expr, $mv_x: expr, $mv_y: expr) => {
        let x = $sprite.x;
        let y = $sprite.y;
        game_engine::update_sprite_position($sprite, x + $mv_x, y + $mv_y);
        if $clear {
            game_engine::clear_screen();
        }
        game_engine::render_sprite($sprite);
    }
}