use std::io;

fn main() {
    let mut _your_name = String::new();
    let mut _age = String::new();
    let mut _department = String::new();
    let mut _class_level = String::new();
    let mut _class_rep = String::new();
    let mut _grade_point_area = String::new();
    let mut _interested_student_council = String::new();
    let _yes = String::new();
    let _no = String::new();

    println!("What is your name?: ");
    io::stdin().read_line(&mut _your_name).expect("Not a valid string");

    println!("How old are you?: ");
    io::stdin().read_line(&mut _age).expect("Not a valid string");
    let _age:f64 = _age.trim().parse().expect("Not a valid number");

    println!("What is your department?: ");
    io::stdin().read_line(&mut _department).expect("Not a valid string");

    println!("What is your class/level?: ");
    io::stdin().read_line(&mut _class_level).expect("Not a valid string");
    let _class_level:f64 = _class_level.trim().parse().expect("Not a valid number");

    println!("Are you a current class rep? \nEnter Yes or No");
    println!("Press 1 for Yes and 0 for No");
    io::stdin().read_line(&mut _class_rep).expect("Not a valid string");
    let _class_rep:f64 = _class_rep.trim().parse().expect("Not a valid number");
    if _class_rep == 1.0{
        println!("You are not eligbible to be in the student council");
    }
    else if _class_rep == 0.0{
        println!("You are eligible to be in the student council");
    }

    println!("Enter your GPA: ");
    io::stdin().read_line(&mut _grade_point_area).expect("Not a valid string");
    let _grade_point_area:f64 = _grade_point_area.trim().parse().expect("Not a valid number");

    println!("Why are you interested in joining the student council?: ");
    io::stdin().read_line(&mut _interested_student_council).expect("Not a valid string");

   if _grade_point_area < 4.0{
    println!("Sorry boss, you can't be a member of the student council. Go and improve your GPA");
   }
   if _age < 18.0 {
    println!("Sorry but you're too young to be in the council");
   }
   if _class_level <= 200.0 || _class_level <= 300.0 || _class_level <= 400.0 {
    println!("Sorry, you have to be in 200lvl or above");
   }

}
