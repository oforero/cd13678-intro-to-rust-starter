
.PHONY: build-c
build-c:
	@echo "Building OpenGL C Wrapper library..."
	make -C game_engine/opengl_libc build-c

.PHONY: run-c
run-c:
	@echo "Running Test Game..."
	make -C game_engine/opengl_libc run-c

.PHONY: build-rust
build-rust:
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

.PHONY: game-rust
game-rust: build-rust
	@echo "Running Rust Simple Test Game..."
	cargo run --manifest-path ./rust_test_game/Cargo.toml
