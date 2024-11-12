use std::env;
use std::path::PathBuf;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
/*    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("opengl_libc/opengl_wrapper_lib/opengl_wrapper_lib.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("-Iopengl_libc/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
*/

    cc::Build::new()
        .file("opengl_libc/opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("opengl_libc/opengl_wrapper_lib")
        // .include("opengl_libc/include")
        .include("/opt/homebrew/include")
        // .link_lib_modifier("cdylib")
        // .flag("-framework")
        // .flag("OpenGL")
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