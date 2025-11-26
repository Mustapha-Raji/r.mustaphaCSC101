use std::fs::OpenOptions;
use std::io::Write;
use std::io;



fn main() {
    let mut file = std::fs::File::create("Data.txt").expect("Create failed");
    file.write_all("Data text file for rust programming\n"
        .as_bytes()).expect("write failed");



    let mut file = OpenOptions::new().append(true).open("Data.txt").expect("Cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("Write failed");
    println!("file append success");

        println!("What do you want to type");
    let mut userinput = String::new();
        io::stdin().read_line(&mut userinput).expect("Invalid Input");





}
