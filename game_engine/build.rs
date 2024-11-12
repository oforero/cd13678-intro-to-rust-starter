
fn main() {

    cc::Build::new()
        .file("opengl_libc/opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("opengl_libc/opengl_wrapper_lib")
        .include("/opt/homebrew/include")
        .flag("-v")
       .compile("opengl_wrapper_lib");

    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/glfw/3.4/lib/"); // Adjust the path if necessary

    // Link against the GLFW static library
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    // Link against Core Graphics
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=AppKit");

}