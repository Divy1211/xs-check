// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the current date and time.
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Write it to the `OUT_DIR` where Cargo can access it.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_date.rs");
    fs::write(&dest_path, format!("const BUILD_DATE: &str = \"{}\";", date)).unwrap();
}