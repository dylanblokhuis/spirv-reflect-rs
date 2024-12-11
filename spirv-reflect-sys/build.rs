#[cfg(feature = "generate_bindings")]
extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=vendor/spirv-reflect/spirv_reflect.h");
    println!("cargo:rerun-if-changed=vendor/spirv-reflect/spirv_reflect.c");

    let mut build = cc::Build::new();

    build.cpp(true).flag("-std=c++11").cpp_link_stdlib("stdc++");

    build.include("src");
    build.file("vendor/spirv-reflect/spirv_reflect.c");

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-sign-compare")
            .flag("-Wno-deprecated")
            .cpp_set_stdlib("c++");
    }

    build.compile("spirv_reflect_cpp");

    generate_bindings("src/bindings.rs");
    println!("cargo:rustc-link-lib=static=spirv_reflect_cpp");
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .header("vendor/spirv-reflect/spirv_reflect.h")
        .size_t_is_usize(true)
        .blocklist_type("__darwin_.*")
        .allowlist_var("SPV.*")
        .allowlist_type("Spv.*")
        .allowlist_function("spv.*")
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_: &str) {}
