use std::io::Read;
use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Your Name and Surname: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _name_surname:String = input1.trim().parse().expect("Not a valid input");

    println!("Enter Your Role: ");
    println!("Type 1 for Office Administrator");
    println!("Type 2 for Project Manager");
    println!("Type 3 for Employee");
    println!("Type 4 for Customer");
    println!("Type 5 for Vendor");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _public_role:i32 = input2.trim().parse().expect("Not a valid number");

    if _public_role == 1{
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

    else if _public_role == 2{
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

    else if _public_role == 3{
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

    else if _public_role == 4{
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

    else if _public_role == 5{
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else{
        println!("Invalid input");
    }

}
