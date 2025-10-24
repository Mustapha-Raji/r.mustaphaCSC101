//Program To solve Quadratic Equation

use std::io;
fn main() {
    //Collecting user Input for a
    println!("What is your value for a?");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("invalid Input");
    let a:f32 = a.trim().parse().expect("Invalid Input");


    //Collecting User Input for b
    println!("What is your value for b?");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Invalid Input");
    let b:f32 = b.trim().parse().expect("Invalid Input");


        //Collecting User Input for c
    println!("What is your value for c?");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Invalid Input");
    let c:f32 = c.trim().parse().expect("Invalid Input");

    //Find discriminant 
    let mut discriminant = (b * b) - 4.0 * a * c;
    let discriminant: f32 = discriminant.sqrt();

    //Find your Solutions for the quadratic equation
    let solution_1 = (-b + discriminant) / (2.0 * a);
    let solution_2 = (-b - discriminant) / (2.0 * a);
    println!("The roots to this quadratic equation are {} and {} respectively", solution_1, solution_2);



    //Find the nature of the roots
    if discriminant < 0.0 {
        println!("This quadratic has no real roots")
    }
    else if discriminant == 0.0 {
        println!("This quadratic has only one real root")
    }
     
    else if discriminant > 0.0 {
        println!("This quadratic has two distinct real roots")
    }
    



}