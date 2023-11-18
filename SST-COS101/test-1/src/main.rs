use std::io;

fn main() {
    let mut _name = String::new();
    let mut _date_of_birth = String::new();
    let mut _email_address = String::new();
    let mut _phone_number = String::new();
    let mut _number_of_siblings = String::new();
    let mut _number_of_children = String::new();
    let mut _medical_diagnosis = String::new(); 
    let mut _village_of_residence = String::new();
    let mut _day = String::new();
    let mut _month = String::new();
    let mut _year = String::new();
    let mut _patient = String::new();
    let mut _age = String::new();
    let mut _amount = String::new();

    println!("Enter your Name: ");
    io::stdin().read_line(&mut _name).expect("Not a valid string");
    
    println!("Enter your Date of Birth: ");
    io::stdin().read_line(&mut _date_of_birth).expect("Not a valid string");
    let _date_of_birth:f64 = _date_of_birth.trim().parse().expect("Not a valid number");
    
    println!("Day");
    io::stdin().read_line(&mut _day).expect("Not a valid string");

    println!("Month");
    io::stdin().read_line(&mut _month).expect("Not a valid string");
    
    println!("Year");
    io::stdin().read_line(&mut _year).expect("Not a valid string");
    let _year:f64 = _year.trim().parse().expect("Not a valid number");
    
    println!("Enter your Email Address: ");
    io::stdin().read_line(&mut _email_address).expect("Not a valid string");

    println!("Enter your Phone Number: ");
    io::stdin().read_line(&mut _phone_number).expect("Not a valid string");
    let _phone_number:f64 = _phone_number.trim().parse().expect("Not a valid number");

    println!("Enter your Number of Siblings: ");
    io::stdin().read_line(&mut _number_of_siblings).expect("Not a valid string");
    let _number_of_siblings:f64 = _number_of_siblings.trim().parse().expect("Not a valid number");

    println!("Enter your Number of Children: ");
    io::stdin().read_line(&mut _number_of_children).expect("Not a valid string");
    let _number_of_children:f64 = _number_of_children.trim().parse().expect("Not a valid number");

    println!("Enter your Medical Diagnosis: ");
    io::stdin().read_line(&mut _medical_diagnosis).expect("Not a valid string");

    println!("Enter your Village of Residence: ");
    io::stdin().read_line(&mut _village_of_residence).expect("Not a valid string");

    println!("HEALTH DIAGNOSIS       AMOUNT(N)     VILLAGE       DISCOUNT");
    println!("Alzheimer              1200000       Akpabom       20%");
    println!("Arrhythmia             550000        Ngbajugi      5%");
    println!("ChronicKidneyDisease   1500000       Atabrikang    15%");
    println!("Diabetis               800000        Okorobilom    10%");
    println!("Arthritis              450000        Emeremen      10%");

    let Alzheimer = 1200000.00;
    let Arrhythmia = 550000.00;
    let ChronicKidneyDisease = 1500000.00;
    let Diabetis = 800000.00;
    let Arthritis = 450000.00;

    if _age = >50.0 && _village_of_residence = Akpabom  && _number_of_children = >4.0{
        let discount = 0.80 * Alzheimer;
        println!("your discounted price is {} naira", discount);
    }
    else{
        println!("The amount is {} naira", Alzheimer);
    }
    if _age >=30.0 && _number_of_siblings = >4.0 && _village_of_residence = Ngbajugi{
        let discount = 0.95 * Arrhythmia;
        println!("Your discounted price is {} naira", discount);
    }  
    else{
        prinln!("The amount is {} naira", Arrhythmia);
    }
    if _age >=40.0 && _number_of_siblings = >3.0 && _number_of_children = >3.0 && _village_of_residence = Atabrikang{
        let discount = 0.85 * ChronicKidneyDisease;
        println!("Your discounted price is {} naira", discount);
    }  
    else{
        prinln!("The amount is {} naira", ChronicKidneyDisease);
    }
    if _age >=28.0 && _age = <45 && _number_of_children = >2.0 && _number_of_children = <4.0 && _village_of_residence = Okorobilom{
            let discount = 0.90 * Diabetis;
            println!("Your discounted price is {} naira", discount);
        }  
    else{
            prinln!("The amount is {} naira", Diabetis);
    }
            if _age >58.0 && _number_of_siblings = >5.0 && _number_of_children = >5.0 && _village_of_residence = Emeremen{
                let discount = 0.90 * Arthritis;
                println!("Your discounted price is {} naira", discount);
            }  
            else{
                prinln!("The amount is {} naira", Arthritis);
            }
}
