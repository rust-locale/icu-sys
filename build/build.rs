extern crate bindgen;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::borrow::{Borrow,ToOwned};

fn main() {
    println!("cargo:rustc-link-lib=icui18n");
    println!("cargo:rustc-link-lib=icuuc");
    println!("cargo:rustc-link-lib=icudata");

    let bindings = bindgen::Builder::default()
        .header("build/icu-std.h")
        .blacklist_type("max_align_t")
        .prepend_enum_name(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings... hm, why?")
        .to_string();

    // detect and remove the suffix
    let detect_re = regex::Regex::new(r"pub fn u_getVersion(.*)\(").unwrap();
    let suffix = detect_re.captures(&*bindings).unwrap().get(1).unwrap().as_str();
    let function_re = regex::Regex::new((r"pub fn ((.*)".to_owned() + suffix + r")\(").borrow()).unwrap();
    let bindings_renamed = function_re.replace_all(bindings.borrow(), "#[link_name = \"$1\"] pub fn $2(");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    File::create(out_dir.join("icu.rs"))
        .expect("Can't create output file.")
        .write_all(bindings_renamed.as_bytes())
        .expect("Can't write output.");
}
