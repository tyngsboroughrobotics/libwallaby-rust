use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    assert!(Command::new("bash")
        .args(&["build.sh"])
        .status()
        .unwrap()
        .success());

    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("libwallaby/lib")
            .to_string_lossy()
    );

    println!("cargo:rustc-link-lib=kipr");

    let ignored_macros = IgnoreMacros(
        vec![
            String::from("FP_INFINITE"),
            String::from("FP_NAN"),
            String::from("FP_NORMAL"),
            String::from("FP_SUBNORMAL"),
            String::from("FP_ZERO"),
        ]
        .into_iter()
        .collect(),
    );

    let bindings = bindgen::Builder::default()
        .header("libwallaby/include/kipr/botball.h")
        .header("libwallaby/include/kipr/wombat.h")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.sh");
    println!("cargo:rerun-if-changed=libwallaby/include/kipr/botball.h");
    println!("cargo:rerun-if-changed=libwallaby/include/kipr/wombat.h");
}

#[derive(Debug)]
struct IgnoreMacros(std::collections::HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
