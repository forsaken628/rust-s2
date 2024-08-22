use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    std::fs::write("out_dir.txt", out_dir).unwrap();

    cmake::Config::new("abseil-src/source")
        .define("CMAKE_CXX_STANDARD", "14")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .build();
    let out_dir = cmake::Config::new("s2-src/source")
        .define("CMAKE_CXX_STANDARD", "14")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=s2");

    env::set_var("PKG_CONFIG_PATH", out_dir.join("lib/pkgconfig"));

    for lib in [
        "absl_flat_hash_set",
        "absl_hash",
        "absl_inlined_vector",
        "absl_span",
        "absl_status",
        "absl_str_format",
        "absl_btree",
        "absl_check",
        "absl_fixed_array",
        "absl_flags",
        "absl_flat_hash_map",
    ] {
        pkg_config::Config::new().statik(true).probe(lib).unwrap();
    }
}
