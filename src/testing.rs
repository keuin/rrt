use std::env;
use std::path::PathBuf;

pub fn path(file_name: &str) -> PathBuf {
    let mut p = PathBuf::new();
    p.push(env::var("CARGO_MANIFEST_DIR").expect("read environment variable"));
    p.push(file_name);
    p
}