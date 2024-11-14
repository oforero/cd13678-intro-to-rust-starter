
.PHONY: build-rust
run-c: build-rust
	@echo "Building Rust Games"
	cargo build --manifest-path ./game_engine/Cargo.toml


.PHONY: test-rust
test-rust: build-rust
	@echo "Running Rust Tests Serially..."
	cargo test --manifest-path ./game_engine/Cargo.toml test --test test_simple_game_loop -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml test --test test_sprite_rendering -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml test --test test_screen_clearing -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml test --test test_key_press -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml test --test test_sprite_position_update -- --nocapture --ignored
	@echo "Tests done"
