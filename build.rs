extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Rebuild if C code changes
    println!("cargo:rerun-if-changed=quirc/lib/");

    let bindings = bindgen::Builder::default()
        .header("quirc/lib/quirc.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}