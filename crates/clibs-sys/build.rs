use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let this_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let top_dir = this_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap();
    let libs_build_dir = top_dir.join("build").join("libs");

    let make_output = Command::new("make")
        .args(["-C", top_dir.to_str().unwrap(), "all"])
        .output()
        .expect("make cmd creation failed");

    if !make_output.status.success() {
        eprintln!(
            "make failed with top-dir:{:?} status: {}",
            top_dir, make_output.status
        );
        eprintln!("stdout:\n{}", String::from_utf8_lossy(&make_output.stdout));
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&make_output.stderr));
        panic!("make failed");
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!(
        "cargo::rustc-link-search=native={}/c-lib",
        libs_build_dir.display()
    );
    println!(
        "cargo::rustc-link-search=native={}/cpp-lib",
        libs_build_dir.display()
    );
    // needed for c++
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo::rustc-link-lib=static=cpp-lib");

    println!("cargo::rustc-link-lib=static=c-lib");

    let c_headers = ["c-lib/c-lib.h"];

    let c_headers = c_headers.map(|header| {
        top_dir
            .join("libs")
            .join(header)
            .to_str()
            .unwrap()
            .to_owned()
    });

    let cpp_headers = ["cpp-lib/cpp-lib.hpp"];
    let cpp_headers = cpp_headers.map(|header| {
        top_dir
            .join("libs")
            .join(header)
            .to_str()
            .unwrap()
            .to_owned()
    });

    let c_bindings = bindgen::Builder::default()
        .headers(c_headers)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to c-lib generate bindings");

    c_bindings
        .write_to_file(out_path.join("c-lib-bindings.rs"))
        .expect("failed to write bindings");

    let cpp_bindings = bindgen::Builder::default()
        .headers(cpp_headers)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate cpp-lib bindings");

    cpp_bindings
        .write_to_file(out_path.join("cpp-lib-bindings.rs"))
        .expect("failed to write cpp-lib bindings");
}
