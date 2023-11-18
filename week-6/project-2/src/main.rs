use std::io;

fn main() {
    let mut x = 0.0;
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Numbers of papers published   Incentive(N) ");
    println!("        3-5                     500,000");
    println!("        6-10                    800,000");
    println!("    less than 3                 100,000");
    println!("   greater than 10              1,000,000");

    println!("Enter your Name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
   
    println!("Enter the numbers of papers published: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let number_of_papers_published:f64 = input2.trim().parse().expect("Not a valid number");

    if x <=500.0{
        if number_of_papers_published >=3.0 && number_of_papers_published <=5.0{
        println!("Your incentive is N500,000");
        }
        if number_of_papers_published >5.0 && number_of_papers_published <10.0{
        println!("Your incentive is N800,000");
        }
        if number_of_papers_published <3.0{
        println!("Your incentive is N100,000");
        }
        if number_of_papers_published >=10.0{
        println!("Your incentive is N1,000,000");
        }
        x = x + 1.0
    
    }
    else{
        println!("The limit for the number of Reseachers who can apply for an incentive has been exceeded");
    }
}
