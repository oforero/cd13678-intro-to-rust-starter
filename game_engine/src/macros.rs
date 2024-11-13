
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