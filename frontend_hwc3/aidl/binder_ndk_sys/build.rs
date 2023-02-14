extern crate bindgen;

use anyhow::Result;
use bindgen::EnumVariation;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn build_stub() -> Result<()> {
    let symbols_str = std::fs::read_to_string("src/symbols.txt")?;
    let symbols = symbols_str.split_terminator('\n');

    // Determine the output directory from the OUT_DIR environment variable
    let out_dir = env::var("OUT_DIR").unwrap();
    let stub_path = Path::new(&out_dir).join("stub.c");

    // Create and write to the stub.c file
    let mut file = File::create(&stub_path).expect("Failed to create stub.c");

    // Write stub functions with void return type and no parameters
    for symbol in symbols {
        writeln!(file, "void {} (void) {{}}", symbol).expect("Failed to write to stub.c");
    }

    let target = std::env::var("TARGET").unwrap();
    let cc = std::env::var(format!("CC_{}", target)).unwrap_or_else(|_| "clang".to_string());
    let cflags = std::env::var(format!("CFLAGS_{}", target)).unwrap_or_default();

    let mut command = Command::new(&cc);

    for flag in cflags.split_whitespace() {
        command.arg(flag);
    }

    // Compile the stub.c file into a shared object (libbinder.so)
    let status = command
        .arg("-shared")
        .arg("-fPIC")
        .arg("-o")
        .arg(format!("{}/libbinder.so", out_dir))
        .arg(&stub_path)
        .status()
        .expect("Failed to compile libbinder.so");

    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }

    // Instruct Cargo to link against libbinder.so and specify the search path
    println!("cargo:rustc-link-lib=binder");
    println!("cargo:rustc-link-search={}", out_dir);

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=src/BinderBindings.hpp");
    println!("cargo:rerun-if-changed=src/symbols.txt");

    build_stub().unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/BinderBindings.hpp")
        .clang_arg("-Isrc/include_cpp")
        .clang_arg("-Isrc/include_ndk")
        .clang_arg("-Isrc/include_platform")
        .clang_arg("-std=c++17")
        .default_enum_style(EnumVariation::Rust { non_exhaustive: true })
        .constified_enum("android::c_interface::consts::.*")
        .allowlist_type("android::c_interface::.*")
        .allowlist_type("AStatus")
        .allowlist_type("AIBinder_Class")
        .allowlist_type("AIBinder")
        .allowlist_type("AIBinder_Weak")
        .allowlist_type("AIBinder_DeathRecipient")
        .allowlist_type("AParcel")
        .allowlist_type("binder_status_t")
        .allowlist_function(".*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
