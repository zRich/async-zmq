extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=libzmq");

    let bindings = bindgen::Builder::default()
        .header("libzmq.h")
        .generate()
        .expect("Unable to generate zmq.rs");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("zmq.rs"))
        .expect("Couldn't write libzmq.rs!");
}
