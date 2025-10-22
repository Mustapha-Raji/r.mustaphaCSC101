use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("\n Enter your height (in centimeters):");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    println!("What is your name?");
    io::stdin().read_line(&mut input2).expect("Not a valid input");


    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person {}", input2);
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall {}", input2);
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf {}", input2);
    }
    else{
        println!("Abnormal height")
    }


}
