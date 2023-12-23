use std::env;
use std::path::PathBuf;

pub fn path(file_name: &str) -> PathBuf {
    let mut p = PathBuf::new();
    let root_dir = env::var("CARGO_MANIFEST_DIR").expect("read environment variable");
    if root_dir.is_empty() {
        panic!("environment CARGO_MANIFEST_DIR is not set");
    }
    p.push(root_dir);
    p.push("resources");
    p.push("test");
    p.push(file_name);
    p
}
