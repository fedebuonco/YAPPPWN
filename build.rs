use std::env;
use std::path::PathBuf;

fn main() {
    // Get the target operating system.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        // Windows-specific configuration search for packet.lib

        // Original path in the project
        let lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("lib");
        let lib_path_str = lib_path.to_str().expect("Failed to convert path to string");

        // Additional path for packet.lib
        let additional_lib_path = PathBuf::from("/tmp/sdk/Lib/x64/");
        let additional_lib_path_str = additional_lib_path
            .to_str()
            .expect("Failed to convert additional path to string");

        // Print the paths to the build script
        println!("cargo:rustc-link-search=native={}", lib_path_str);
        println!("cargo:rustc-link-search=native={}", additional_lib_path_str);
        println!("cargo:rustc-link-lib=Packet");
    } else if target_os == "macos" {
        // macOS-specific configuration
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }
}
