use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=cpp_bindings/PlaneInfo.cpp");
    println!("cargo:rerun-if-changed=cpp_bindings/plane_layouts.h");
    println!("cargo:rerun-if-changed=cpp_bindings/libui_stub.cpp");
    println!("cargo:rerun-if-changed=cpp_bindings/libui_stub.h");
    println!("cargo:rerun-if-changed=build.rs");

    let target = std::env::var("TARGET").unwrap();

    let cxx = std::env::var(format!("CXX_{}", target)).unwrap_or_else(|_| "clang++".to_string());
    let cxxflags = std::env::var(format!("CXXFLAGS_{}", target)).unwrap_or_default();
    let ar = std::env::var(format!("AR_{}", target)).unwrap_or_else(|_| "llvm-ar".to_string());

    let mut command = std::process::Command::new(&cxx);

    // Add flags from CXXFLAGS
    for flag in cxxflags.split_whitespace() {
        command.arg(flag);
    }

    // Build libui stub
    let status = command
        .arg("-fvisibility=default")
        .arg("-shared")
        .arg("-stdlib=libc++") // libc++ is needed to produce correct mangled symbols that is compatible with AOSP's libui.so
        .arg("-static-libstdc++")
        .arg("-fPIC")
        .arg("-o")
        .arg(format!("{}/libui.so", out_dir))
        .arg("cpp_bindings/libui_stub.cpp")
        .status()
        .expect("Failed to compile libui_stub.so");

    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }

    println!("cargo:rustc-link-lib=ui");
    println!("cargo:rustc-link-search={}", out_dir);

    let mut command = std::process::Command::new(&cxx);

    // Add flags from CXXFLAGS
    for flag in cxxflags.split_whitespace() {
        command.arg(flag);
    }

    // Build libplaneinfo
    let status = command
        .arg("-c")
        .arg("-stdlib=libc++") // libc++ is needed to produce correct mangled symbols that is compatible with AOSP's libui.so
        .arg("-static-libstdc++")
        .arg("-o")
        .arg(format!("{}/libplaneinfo.o", out_dir))
        .arg("cpp_bindings/PlaneInfo.cpp")
        .status()
        .expect("Failed to compile libplaneinfo.o");

    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }

    let status = std::process::Command::new(&ar)
        .arg("rcs")
        .arg(format!("{}/libplaneinfo.a", out_dir))
        .arg(format!("{}/libplaneinfo.o", out_dir))
        .status()
        .expect("Failed to archive libplaneinfo.o");

    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }

    println!("cargo:rustc-link-lib=planeinfo");
    println!("cargo:rustc-link-search={}", out_dir);

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        println!("cargo:rustc-link-lib=c++_static");
    } else {
        println!("cargo:rustc-link-lib=c++");
    }
}
