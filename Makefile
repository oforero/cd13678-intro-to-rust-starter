
.PHONY: test-rust
test-rust:
	@echo "Running Rust Tests Serially..."
	cargo test --manifest-path ./game_engine/Cargo.toml tests::test_simple_game_loop -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml tests::test_sprite_rendering -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml tests::test_screen_clearing -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml tests::test_key_presses -- --nocapture --ignored
	cargo test --manifest-path ./game_engine/Cargo.toml tests::test_sprite_position_update -- --nocapture --ignored
