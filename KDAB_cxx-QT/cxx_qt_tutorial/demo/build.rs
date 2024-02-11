// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .compile("demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
