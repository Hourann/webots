use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=../../../lib");
    println!("cargo:rustc-link-lib=Controller");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .header("wrapper.h")
        .clang_arg("-I../../../include/controller/c")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
