extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        includes: &[],
        input: &["foobar.proto"],
    }).expect("Failed to generate Rust src");
}
