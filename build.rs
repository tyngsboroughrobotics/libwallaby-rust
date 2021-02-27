use std::env;
use std::path::PathBuf;
use std::{process::Command, str};

fn main() {
    let llvm_config_out = Command::new("llvm-config")
        .args(&["--ldflags", "--system-libs", "--libs", "core"])
        .output()
        .expect("failed to execute llvm-config");

    let llvm_clang_args = llvm_config_out
        .stdout
        .split(|byte| byte.is_ascii_whitespace())
        .map(|arg| str::from_utf8(arg).unwrap());

    println!("cargo:rustc-link-lib=botball");
    println!("cargo:rerun-if-changed=include/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_args(llvm_clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
