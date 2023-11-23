use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();
    let mut input13 = String::new();

    //empty array
    let mut myarr:Vec<String> = Vec::new();

    println!("How many siblings do you have?(Please numbers only e.g 2,3): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _number_of_siblings:i32 = input1.trim().parse().expect("Not a valid number");
    //pushing element into array
    myarr.push(input1.to_string());

    for i in 0.._number_of_siblings {

    println!("Enter First Name(s): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    //pushing element into array
    myarr.push(input2.to_string());

    println!("Enter Age(s)(Please numbers only e.g 14,25): ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _age:f64 = input3.trim().parse().expect("Not a valid number");
    //pushing element into array
    myarr.push(input3.to_string());

    if _age > 18.0 {
        println!("Is He/She Married or Single?: ");
        println!("Type 1 for Married and 0 for Single");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let _married_or_single:f64 = input4.trim().parse().expect("Not a valid number");
        //pushing element into array
        myarr.push(input4.to_string());

        if _married_or_single == 0.0 {
            println!("Is He/She a Worker or Student?: ");
            println!("Type 1 for Worker and 0 for Student");
            io::stdin().read_line(&mut input5).expect("Not a valid string");
            let _worker_or_student:f64 = input5.trim().parse().expect("Not a valid number");
            //pushing element into array
            myarr.push(input5.to_string());

        if _worker_or_student == 0.0 {
            println!("Enter Name of University: ");
            io::stdin().read_line(&mut input6).expect("Not a valid string");        
            //pushing element into array
            myarr.push(input6.to_string());

            println!("Enter Course of Study: ");
            io::stdin().read_line(&mut input7).expect("Not a valid string"); 
            //pushing element into array
            myarr.push(input7.to_string());
            
        }
        }
        else if _married_or_single == 1.0{
            println!("Does He/She have Children in their marriage?: ");
            println!("Type 1 for Yes and 0 for No");
            io::stdin().read_line(&mut input8).expect("Not a valid string");
            let _children_in_marriage:f64 = input8.trim().parse().expect("Not a valid number");
            //pushing element into array
            myarr.push(input8.to_string());
            
            println!("What is the  name of the city that His/Her family lives in?: ");
            io::stdin().read_line(&mut input9).expect("Not a valid string");
            //pushing element into array
            myarr.push(input9.to_string());
        }
    
    }
    else if _age < 18.0 {
        println!("Has He/She wrritten WAEC?: ");
        println!("Type 1 for Yes and 0 for No");
        io::stdin().read_line(&mut input10).expect("Not a valid string");
        let _written_waec:f64 = input10.trim().parse().expect("Not a valid number");
        //pushing element into array
        myarr.push(input10.to_string());
        
        if _written_waec == 0.0{
    
        println!("Enter name of Secondary School: ");
        io::stdin().read_line(&mut input11).expect("Not a valid string");
        //pushing element into array
        myarr.push(input11.to_string());

        println!("Enter Current Class Level: ");
        io::stdin().read_line(&mut input12).expect("Not a valid string");
        //pushing element into array
        myarr.push(input12.to_string());
    
        }
        if _written_waec == 1.0 {
            println!("Sorry but your data is not defined in the database. Please input your status");
            io::stdin().read_line(&mut input13).expect("Not a valid string");
            //pushing element into array
            myarr.push(input13.to_string());

        }
    }
    for val in myarr.iter(){
        println!("{:?}",val);
        }
    }
}