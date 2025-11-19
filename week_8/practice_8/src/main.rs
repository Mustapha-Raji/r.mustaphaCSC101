fn main() {
    //IInitialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original Tupe = {:?}", mountain_heights);

    //change 3rd 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}",mountain_heights);
}
