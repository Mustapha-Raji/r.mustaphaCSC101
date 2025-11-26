use std::fs;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("deleted_file").expect("Create failed");
    file.write_all("Mustapha's delete file\n"
        .as_bytes()).expect("write failed");

    fs::remove_file("deleted_file").expect("could not remove file");
    println!("file is removed");


}