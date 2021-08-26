extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=libzmq");

    let bindings = bindgen::Builder::default()
        .header("libzmq.h")
        .generate()
        .expect("Unable to generate libzmq.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libzmq.rs"))
        .expect("Couldn't write libzmq.rs!");
}
