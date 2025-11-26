use std::io::Read;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("welcome_message.txt").expect("Create Failed");
    file.write_all("Welcome to Mustapha's Rust program\n"
        .as_bytes()).expect("write failed");


    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
