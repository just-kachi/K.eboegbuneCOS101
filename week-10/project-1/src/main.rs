struct Laptop {
    Hp:u32, Ibm:u32, Toshiba:u32, Dell:u32,
}

impl Laptop {
    fn sum(&self)->u32 {
        self.Hp + self.Ibm + self.Toshiba + self.Dell
    }
}

fn main() {

    let total = Laptop {
        Hp: 650000 * 3,
        Ibm: 755000 * 3,
        Toshiba: 550000 * 3,
        Dell: 850000 * 3,
    };
    println!("The total cost of 3 HP laptops is {}",total.Hp);
    println!("The total cost of 3 HP laptops is {}",total.Ibm);
    println!("The total cost of 3 HP laptops is {}",total.Toshiba);
    println!("The total cost of 3 HP laptops is {}",total.Dell);
    println!("The total sum of your purchases amounts to {}",total.sum());
}
