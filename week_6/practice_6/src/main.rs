fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; //2 & n3 reference is passed

    //About Elctrical/Electronic
    println!("\n The {} is informed by the aspiration to train electrical/electronic egineering professionals is in the areas of design, building and maintenance of electrical control system,", n4);


    let w1 = "computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;
    println!();
    println!("{} is aimed at developing competent, creative, innovative, entrepreneural and ethically-minded persons, capable of creating valye in the diverse fields of Computer Science. ",w3);
}
