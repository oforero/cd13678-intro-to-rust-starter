OS := $(shell uname)
 ifeq ($(OS), Darwin)
        OS := mac
 else ifeq ($(OS), Linux)
        OS := linux
 else
        $(info Warning: unknown OS, assuming linux)
        OS := linux
 endif

OUTPUT_DIR = ./c_output

OPENGL_WRAPPER_LIB_DIR = ./opengl_wrapper_lib
OPENGL_WRAPPER_LIB_SRC = $(OPENGL_WRAPPER_LIB_DIR)/opengl_wrapper_lib.c
OPENGL_WRAPPER_LIB_OBJ = $(OUTPUT_DIR)/opengl_wrapper_lib.o
OPENGL_WRAPPER_LIB = openglwrapper
OPENGL_WRAPPER_LIB_SO = $(OUTPUT_DIR)/lib$(OPENGL_WRAPPER_LIB).so

TEST_GAME_DIR = ./c_test_game
TEST_GAME_SRC = $(TEST_GAME_DIR)/test_game.c
TEST_GAME_EXE = $(OUTPUT_DIR)/test_game_exe

ifeq ($(OS), linux)
    LINK_OPENGL = -lGL
    CC=gcc
else ifeq ($(OS), mac)
	PREFIX = $$(brew --prefix)
	INCLUDE = -I$(PREFIX)/include
	LIB = -L$(PREFIX)/lib
	CC=gcc $(INCLUDE) $(LIB)
    LINK_OPENGL = -framework OpenGL
endif

.PHONY: build-c
build-c:
	@echo "Building OpenGL Wrapper library..."
	$(CC) -c -fPIC $(OPENGL_WRAPPER_LIB_SRC) -o $(OPENGL_WRAPPER_LIB_OBJ) # Compile source file
	$(CC) -shared -o $(OPENGL_WRAPPER_LIB_SO) $(OPENGL_WRAPPER_LIB_OBJ) -lglfw  $(LINK_OPENGL) # Create shared library

.PHONY: run-c
run-c: build-c
	@echo "Running Test Game..."
	$(CC) $(TEST_GAME_SRC) -o $(TEST_GAME_EXE) -L$(OUTPUT_DIR) -l$(OPENGL_WRAPPER_LIB) -Wl,-rpath,$(OUTPUT_DIR) # Compile source file
	$(TEST_GAME_EXE) # Run the test game

