use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("test_cases.rs");
    let input_vec = Command::new("ls").args(&["problems"]).output().unwrap().stdout;
    let input = String::from_utf8(input_vec).expect("Found invalid UTF-8");
    let input: &str = &input;
    fs::write(
	&dest_path,
	String::from(format!("pub fn test_message() -> &'static str {{
        \"{}\"
}}
", input))).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
