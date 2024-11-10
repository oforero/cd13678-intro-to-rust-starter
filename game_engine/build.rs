fn main() {
    cc::Build::new()
        .file("./opengl_libc/opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("./opengl_libc/opengl_wrapper_lib/")
        .compile("opengl_wrapper_lib.o");
}