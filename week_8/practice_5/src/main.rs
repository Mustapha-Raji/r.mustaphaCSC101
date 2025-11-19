fn main() {
    // Create an empty of vector "City"

    let mut city : Vec<String> =Vec::new();
    //Print City VEctor
    println!("The city vector has element {}",city.len());
    //Push new elements into
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid Input");
    for count in 0 ..city_num {
        let mut input2 = String::new();
        println!("Enter city {}",count +1);
        std::io::stdin().read_line(&mut input2).expect("Invalid Input");
        let new_city:String = input2.trim().parse().expect("Invalid Input");
        city.push(new_city);

    }
    print!("Your prefferal cities are: \n");
    let mut count = 1;
    //loop to iterate elements in vector
    for i in city{
        //iterating through i on the vector
        println!("{} {}",count, i);
        count +=1;
    }
}
