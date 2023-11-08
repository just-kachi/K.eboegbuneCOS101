

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let years_of_experience:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40.0 && years_of_experience >= 8.0
    {
        println!("Your incentive is N1,560,000");
    } 
    else if age >= 30.0 && age < 40.0 && years_of_experience >= 8.0
    {
        println!("Your incentive is N1,480,00");
    } 
    else if age < 30.0 && years_of_experience >= 8.0
    {
        println!("Your incentive is N1,300,000");
    } 
    else
    {
        println!("Your incentive is N100,000")
    }
}