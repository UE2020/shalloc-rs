use std::env;
use std::path::PathBuf;

fn main() {
    // Generate bindings for the C header
    let bindings = bindgen::Builder::default()
        .header("alloc/include/alloc_ext.h") // Path to your C header
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the appropriate location in OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("alloc/src/alloc.c")
        .file("alloc/src/debug.c")
        .compile("shadalloc");

    println!("cargo:rerun-if-changed=alloc/include/alloc_ext.h");
    println!("cargo:rerun-if-changed=alloc/src/alloc.c");
}
