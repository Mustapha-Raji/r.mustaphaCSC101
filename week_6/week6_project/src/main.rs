use std::io;

fn main() {
    println!("Welcome to Mustapha's Diner!!!");
    println!("Take a look at our menu please.");
        println!(" Menu                                 |   Price
               P = Poundo Yam / Edinkaiko Soup       |      N3,200
               F = Fried Rice & Chicken              |      N3,000
               A = Amala & Ewedu Soup                |      N2,500  
               E = Eba & Egusi Soup                  |      N2,000
               W = White Rice & Stew                 |      N2,500
               D = Done ordering");
    let mut total: f32 = 0.00;

    loop{
        println!("What do you want to order?");
        //Taking user input/ order 
        let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Invalid Input");
            user_input = user_input.trim().to_string();
        let mut price: f32 = 0.00;

        if user_input == "P".to_lowercase() {
            price = 3200.00;
        }
        if user_input == "F".to_lowercase() {
            price = 3000.00;
        }
        if user_input == "A".to_lowercase() {
            price = 2500.00;
        }
        if user_input == "E".to_lowercase(){
            price = 2000.00;
        }
        if user_input == "W".to_lowercase(){
            price = 2500.00
        }
        if user_input == "D".to_lowercase(){
            break;
        }
        // Collecting quantity
        println!("How many portions?");
        let mut quantity_ordered = String::new();
            io::stdin().read_line(&mut quantity_ordered).expect("Invalid Input");
            let quantity_ordered: f32 = quantity_ordered.trim().parse().expect("Invalid Input");
        // Computing the total 
        total = price * quantity_ordered;
        println!("Your total is {}",total);
        println!("Thank you so much for your patronage");

        if total >= 10_000.00 {
            let discounted_total = total * 0.05;
            total -= discounted_total;
            println!("Amount discunted {}",discounted_total);
            println!("Your Total: {}", total);
            println!("Thank you so much for your patronage");
        }
    }
}