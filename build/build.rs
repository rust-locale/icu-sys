extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=icui18n");
    println!("cargo:rustc-link-lib=icuuc");
    println!("cargo:rustc-link-lib=icudata");

    let bindings = bindgen::Builder::default()
        .header("build/icu-std.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings... hm, why?");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("icu.rs"))
        .expect("Unable to write bindings... hm, why?");
}
