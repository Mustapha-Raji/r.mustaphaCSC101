use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::create("Data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming \n"
        .as_bytes()).expect("write failed");
    
    let mut file = std::fs::File::open("Data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}