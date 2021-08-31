extern crate bindgen;

use std::{env, path::{Path, PathBuf}};

fn main() {
    println!("cargo:rustc-link-lib=zmq");
    // println!("cargo:rustc-link-lib=libzmq");
    println!("cargo:rustc-link-search=all={}", "/usr/local/lib");
    println!("cargo:include={}", "/usr/local/include");

    let bindings = bindgen::Builder::default()
        .header("libzmq.h")
        .generate()
        .expect("Unable to generate zmq.rs");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("zmq.rs"))
        .expect("Couldn't write libzmq.rs!");
}
