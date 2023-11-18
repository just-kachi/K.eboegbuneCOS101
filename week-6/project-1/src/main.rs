use std::io;

fn main() {
    let mut x = 0.0;
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Enter Name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Are you a current class representative? \nEnter Yes or No: ");
    println!("Type 1 for Yes and Type 0 for No");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _class_rep:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter Email: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter Department: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    
    println!("Enter Your Level(100,200): ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _class_level:f64 = input5.trim().parse().expect("Not a valid number");

    println!("Enter CGPA: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let _cgpa:f64 = input6.trim().parse().expect("Not a valid number");

    println!("Enter State of Origin: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");

    if x <= 150.0 {
        if _cgpa > 4.0 && _class_rep == 1.0 && _class_level > 100.0{
            println!("You can vote!");
            println!("Your details {},{},{},{},{},{}", input1, input3, input4, input5, input6, input7);  
            x = x + 1.0
        }
        else {
            println!("You can't vote!");
        }
    } 
    else { 
        println!("The limit of people who can vote has been exceeded");
    }

}
