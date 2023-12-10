use std::io::Write;
use std::io;

fn main() {

    let mut usersname : Vec<String> = Vec::new();
    let mut usersmatriculationnumber : Vec<String> = Vec::new();
    let mut userscourseofstudy : Vec<String> = Vec::new();
    let mut userslevel : Vec<String> = Vec::new();

    let mut input5 = String::new();
    println!("(ADMIN ONLY)How many student's personal details do you want to input into the system:");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _student_personal_detail:i32 = input5.trim().parse().expect("Not a valid number");

    for val in 0.._student_personal_detail {

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();

        println!("Enter Your Name: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");

        println!("Enter Your Matriculation Number: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");

        println!("Enter Your Course of Study: ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");

        println!("Enter Your Level: ");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let _level:i64 = input4.trim().parse().expect("Not a valid number");

        usersname.push(input1.to_string());
        usersmatriculationnumber.push(input2.to_string());
        userscourseofstudy.push(input3.to_string());
        userslevel.push(input4.to_string());

    }

    let mut username : String = usersname.join("");
    let mut usersmatriculationnumber : String = usersmatriculationnumber.join("");
    let mut userscourseofstudy : String = userscourseofstudy.join("");
    let mut userslevel : String = userslevel.join("");

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(username.as_bytes()).expect("write failed");
    file.write_all(usersmatriculationnumber.as_bytes()).expect("write failed");
    file.write_all(userscourseofstudy.as_bytes()).expect("write failed");
    file.write_all(userslevel.as_bytes()).expect("write failed");
    println!("\nData written to file.");

    println!("These are your Personal Details:");
    println!("Name: {:?}", usersname);
    println!("Matriculation Number: is {:?}", usersmatriculationnumber);
    println!("Course of Study: {:?}", userscourseofstudy);
    println!("Class Level: {:?}", userslevel);

}
