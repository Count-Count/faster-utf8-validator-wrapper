use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("../faster-utf8-validator-c/z_validate.c")
        .define("AVX2", "1")
        .flag("-march=native")
        .warnings(true)
        .warnings_into_errors(true)
        .opt_level(3)
        .compile("libzvalidate.a");

    eprintln!("bindings");
    let bindings = bindgen::Builder::default()
        .clang_args(&["../faster-utf8-validator-c/z_validate.c", "-DAVX2"])
        .whitelist_function("z_validate_utf8_avx2")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");
    eprintln!("bindings done");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
