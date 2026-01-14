use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let max_addr = env::var("MAX_ADDR")
        .unwrap_or_else(|_| "0x80000000".to_string());

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");

    fs::write(
        &dest_path,
        format!("pub const MAX_ADDR: usize = {};", max_addr)
    ).unwrap();

    println!("cargo:rerun-if-env-changed=MAX_ADDR");
}
