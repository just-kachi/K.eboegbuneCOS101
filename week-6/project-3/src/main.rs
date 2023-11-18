use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new(); 
    
    println!("Enter your value for M: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _value_for_m:i64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your N(nth term): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let n:i64 = input2.trim().parse().expect("Not a valid number");

    let x = n + 1;

    for i in 1..x {
        println!("{} x {} = {}",_value_for_m, i,_value_for_m * i);
    }
}
