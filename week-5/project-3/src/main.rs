use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Menu: ");
    println!("Pounded Yam/Edinkaiko Soup - N3200");
    println!("Fried Rice & Chicken - N3000");
    println!("Amala & Ewedu Soup - N2500");
    println!("Eba and Egusi Soup - N2000");
    println!("White Rice & Stew - N2500");

    println!("Enter Amount of Pounded yam/Edinkaiko soup: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let pounded_yam_edinkaiko_soup:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter Amount of Fried Rice & Chicken: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let fried_rice_chicken:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter Amount of Amala & Ewedu Soup: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let amala_ewedu_soup:f64 = input3.trim().parse().expect("Not a valid number");

    println!("Enter Amount of Eba and Egusi Soup: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let eba_and_egusi_soup:f64 = input4.trim().parse().expect("Not a valid number");

    println!("Enter Amount of White Rice & Stew: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let white_rice_stew:f64 = input5.trim().parse().expect("Not a valid number");

    if pounded_yam_edinkaiko_soup > 50.0 || fried_rice_chicken > 50.0 || amala_ewedu_soup > 50.0 || eba_and_egusi_soup > 50.0 || white_rice_stew > 50.0 {
        println!("Please reduce the amount; diabetis is real");
    }
    else{
        let P = pounded_yam_edinkaiko_soup * 3200.00;
        let F = fried_rice_chicken * 3000.00;
        let A = amala_ewedu_soup * 2500.00;
        let E = eba_and_egusi_soup * 2000.00;
        let W = white_rice_stew * 2500.00;
        //T stands for total order
        let T = P + F + A + E + W; 

        if T >= 10000.00 {
            let discount = 0.95 * T;
        println!("Your discounted price is {} naira", discount);
        }
        else {
            println!("Your total price is {} naira", T);
        }
    }
}
