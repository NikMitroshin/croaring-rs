extern crate cc;

use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .flag_if_supported("-std=c11")
        .flag_if_supported("-march=generic")
        .flag_if_supported("-O3")
        .file("CRoaring/roaring.c")
        .compile("roaring");

    let bindings = bindgen::Builder::default()
        .blacklist_type("max_align_t")
        .header("CRoaring/roaring.h")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("croaring-sys.rs"))
        .expect("Couldn't write bindings!");
}

