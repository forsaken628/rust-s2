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
    let mut builder = Builder::default()
        .size_t_is_usize(true)
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++14",
            &("-I".to_string() + include_path.as_os_str().to_str().unwrap()),
            "-fretain-comments-from-system-headers",
        ])
        .ctypes_prefix("libc")
        .no_convert_floats()
        .wrap_static_fns(true)
        .raw_line("#![allow(nonstandard_style)]\n#![allow(improper_ctypes)]");

    for s in ["s2/s2latlng.h", "s2/s2polyline.h"] {
        let header = include_path.join(s);
        builder = builder.header(header.as_os_str().to_str().unwrap());
    }

    for s in [
        "std::string",
        "std::unique_ptr",
        "std::vector",
        "absl::Nullable",
        "Vector2",
        "Vector3",
    ] {
        builder = builder.opaque_type(s);
    }

    for s in ["S2LatLng", "S2Polyline"] {
        builder = builder.allowlist_type(s);
    }

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Unable to write bindings to file");

    println!("Bindings generated successfully; please review the results");
}
