use std::io;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
fn main() {
    println!("Welcome to Nigerian Brewery Limited, explore our rich heritage and exceptional portfolio crafted since 1946.");
    println!("Lager         Stout       Non-Alcoholic");
    let mut file = File::create("table.txt").expect("Failed to create table");
    writeln!(file, "Lager     Stout    Non-Alcoholic");
    loop {
        println!("Name of the drink? ");
    let mut name_of_drink = String::new();
        io::stdin()
        .read_line(&mut name_of_drink)
        .expect("Invalid Input");
    println!("Category Of Drink: Lager(L), Stout(S), Non Alcoholic(N)");  
    let mut category = String::new();
        io::stdin()
        .read_line(&mut category)
        .expect("Invalid Drink Catergory");
    let category: &str = category.trim();
    //make the columns for lager, Stout and Non- Alcoholic empty
    let mut lagerc = String::new();
    let mut stoutc = String::new();
    let mut non_alcoholic = String::new();
    match category {
        "L" => lagerc = name_of_drink,
        "S" => stoutc = name_of_drink,
        "N" => non_alcoholic = name_of_drink,
        _=> {
            println!("Unknown letter/Category");
            continue;

         }
        };

        //Create file
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)//
        .open("table.txt")
        .expect("cannot open file");
        let _ = writeln!(file,"Lager        Stout       Non-Alcoholic
                        {}          {}          {}",lagerc, stoutc, non_alcoholic );


    println!("Will that be all? (Y) (N)");
    let mut is_done = String::new();
        io::stdin()
        .read_line(&mut is_done)
        .expect("invalid input");
    let is_done = is_done.trim();

        if is_done == "Y" {
            break;
        }
        if is_done == "N" {
            continue;
        }


    };




}
