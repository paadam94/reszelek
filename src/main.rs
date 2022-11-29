include!(concat!(env!("OUT_DIR"), "/test_cases.rs"));

use phf::phf_map;

static FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "test1" => include_str!("test/test1.txt"),
    "test2" => include_str!("test/test2.txt"),
};

fn main() {
    let hello: &str = include_str!("test/test1.txt");
    println!("{}", FILES.get("test1").unwrap());
    println!("{}", test_message());
}
