use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let bindings = bindgen::Builder::default()
        .header("alloc/include/alloc/base.h")
        .clang_arg("-Ialloc/include")
        .clang_arg("-std=c23")
        .clang_arg("-march=x86-64-v3")
        .clang_arg("-DNDEBUG")
        .clang_arg("-D_GNU_SOURCE")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    builder
        .include("alloc/include")
        .define("_GNU_SOURCE", None)
        .define("NDEBUG", None)
        .flag_if_supported("-std=c23")
        .flag_if_supported("-march=x86-64-v3")
        .flag_if_supported("-O3")
        .flag_if_supported("-fvisibility=hidden");

    let src_dir = "alloc/src";
    let entries = fs::read_dir(src_dir).expect("Failed to read src directory");

    for entry in entries {
        let path = entry.expect("Failed to read entry").path();
        if path.extension().map_or(false, |ext| ext == "c") {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            match file_name {
                "linux.c" if target_os != "linux" => continue,
                "windows.c" if target_os != "windows" => continue,
                "macos.c" if target_os != "macos" => continue,
                _ => builder.file(&path),
            };
        }
    }

    builder.compile("shalloc");

    println!("cargo:rerun-if-changed=alloc/include");
    println!("cargo:rerun-if-changed=alloc/src");
}
