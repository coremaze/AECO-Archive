fn main() {
    let src = ["compression/src/compression.cpp"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("compression/include")
        .flag("-Wno-parentheses")
        .flag("-Wno-sign-compare");
    build.compile("libecocompression");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("compression/include/compression.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/compression/bindings.rs")
        .expect("Couldn't write bindings!");
}
