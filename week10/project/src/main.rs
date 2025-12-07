struct laptop {
    dell:u64,
    toshiba:u64,
    ibm:u64,
    hp:u64
}

impl laptop {
    fn total(&self)->u64 {
        (self.dell * 3) + (self.toshiba * 3) + (self.ibm * 3) + (self.hp * 3)
    }
}

fn main() {
    let price = laptop {
        dell: 850_000,
        toshiba: 550_000,
        ibm: 755_000,
        hp: 650_000
    };

    println!("Hello!\n");
    println!("Price of dell: {} \n
              Price of toshiba: {} \n
              Price of ibm: {} \n
              Price of hp: {} \n
               The total cost of purchasing three from each brand is: {}"
               ,price.dell,price.toshiba,price.ibm,price.hp,price.total());
}
