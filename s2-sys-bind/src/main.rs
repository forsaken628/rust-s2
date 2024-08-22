use bindgen::Builder;
use std::path::{Path, PathBuf};

fn main() {
    let data = std::fs::read("sys/out_dir.txt").unwrap();
    let data = String::from_utf8(data).unwrap();
    let include_path = PathBuf::from(data).join("include");
    let out_path = PathBuf::from("./sys/src/s2.rs");
    write_bindings(include_path, &out_path)
}

fn write_bindings(include_path: PathBuf, out_path: &Path) {
    Builder::default()
        .size_t_is_usize(true)
        .header(
            include_path
                .join("s2/s2latlng.h")
                .as_os_str()
                .to_str()
                .unwrap(),
        )
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++14",
            &("-I".to_string() + include_path.as_os_str().to_str().unwrap()),
            "-fretain-comments-from-system-headers",
        ])
        .ctypes_prefix("libc")
        .opaque_type("std::string")
        .opaque_type("Vector2")
        .opaque_type("Vector3")
        .allowlist_type("S2LatLng")
        .no_convert_floats()
        .wrap_static_fns(true)
        //.generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Unable to write bindings to file");

    println!("Bindings generated successfully; please review the results");
}
