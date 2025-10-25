use std::io;

fn main() {
    // Collecting age
    println!("How old are you?");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Invalid Input");
        let age:u8 = age.trim().parse().expect("Invalid Input");
    // Checking experience
    println!("Do you have experience?");
    let mut experience = String::new();
        io::stdin()
        .read_line(&mut experience)
        .expect("Invalid Input");
    let experience = experience.trim().to_lowercase();

    let has_experience: bool = experience == "yes";

    //Determining Annual Incentive
    if age>= 40 && has_experience {
        println!("Your annual incentive is 1,560,000");
    }
    else if age >= 30 && age <40  && has_experience{
        println!("Your Annual Incentive is 1,480,000");
    }
    else if age < 28 && has_experience {
        println!("Your Annual incentive is  1,300,00 ");
    }
    else {
        println!("Your incentive is 100,000");
    }



}
