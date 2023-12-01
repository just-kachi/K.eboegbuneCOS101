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

    let _aps_1_2 = vec!["Intern", "blank", "Paralegal", "Placement"];
    let _aps_3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let _aps_5_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"];
    let _els_8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let _els_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let _ses = vec!["CEO", "Dean", "Partner", "Principal"];
    
    println!("Enter Your Name and Surname: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _name_surname:String = input1.trim().parse().expect("Not a valid input");

    println!("Enter Your Age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _age:String = input2.trim().parse().expect("Not a valid input");

    println!("Enter Your Email: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _email:String = input3.trim().parse().expect("Not a valid number");

    println!("Enter Your Telephone Number: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let _telephone_number:f64 = input4.trim().parse().expect("Not a valid number");

    println!("Enter Your Years of Work Experience: ");
    io::stdin().read_line(&mut input10).expect("Not a valid string");
    let _work_experience:f64 = input10.trim().parse().expect("Not a valid number");

    println!("Enter Your Public Servant Role: ");
    println!("Type 1 for Office Administrator");
    println!("Type 2 for Academic");
    println!("Type 3 for Lawyer");
    println!("Type 4 for Teacher");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _public_servant_role:f64 = input5.trim().parse().expect("Not a valid number");

    if _public_servant_role == 1.0 {
        println!("Enter Your Job Description in your Public Servant Role: ");
        println!("Type 1 for Intern");
        println!("Type 2 for Administrator");
        println!("Type 3 for Senior Administrator");
        println!("Type 4 for Office Manager");
        println!("Type 5 for Director");
        println!("Type 6 for CEO");
        io::stdin().read_line(&mut input6).expect("Not a valid string");
        let _job_description:f64 = input6.trim().parse().expect("Not a valid number");

        if _work_experience >= 1.0 && _work_experience <=2.0 && _job_description == 1.0 {
            println!("{}, Your position is APS 1-2",_name_surname);
        }
        else {
            println!("You don't hold any position");
        }
        if _work_experience >=3.0 && _work_experience <=5.0 && _job_description == 2.0{
            println!("{}, Your position is APS 3-5",_name_surname );
        }
        if _work_experience >5.0 && _work_experience <8.0 && _job_description == 3.0{
            println!("{}, Your position is APS 5-8",_name_surname);
        }
        if _work_experience >=8.0 && _work_experience <=10.0 && _job_description == 4.0{
            println!("{}, Your position is EL1 8-10",_name_surname);
        }
        if _work_experience >10.0 && _work_experience <=13.0 && _job_description == 5.0{
            println!("{}, Your position is EL2 10-13",_name_surname);
        }
        if _work_experience >=14.0 && _job_description == 6.0{
            println!("{}, Your position is SES",_name_surname);
        } 
    
        

    }
    if _public_servant_role == 2.0 {
        println!("Enter Your Job Description in your Public Servant Role: ");
        println!("Type 1");
        println!("Type 2 for Research Assistant");
        println!("Type 3 for PhD Candidate");
        println!("Type 4 for Post-Doc Researcher");
        println!("Type 5 for Senior Lecturer");
        println!("Type 6 for Dean");
        io::stdin().read_line(&mut input7).expect("Not a valid string");
        let _job_description:f64 = input7.trim().parse().expect("Not a valid number");
        
        if _work_experience >= 1.0 && _work_experience <=2.0 && _job_description == 1.0 {
            println!("{}, Your position is APS 1-2",_name_surname);
        }
        else {
            println!("You don't hold any position");
        }
        if _work_experience >=3.0 && _work_experience <=5.0 && _job_description == 2.0{
            println!("{}, Your position is APS 3-5",_name_surname );
        }
        if _work_experience >5.0 && _work_experience <8.0 && _job_description == 3.0{
            println!("{}, Your position is APS 5-8",_name_surname);
        }
        if _work_experience >=8.0 && _work_experience <=10.0 && _job_description == 4.0{
            println!("{}, Your position is EL1 8-10",_name_surname);
        }
        if _work_experience >10.0 && _work_experience <=13.0 && _job_description == 5.0{
            println!("{}, Your position is EL2 10-13",_name_surname);
        }
        if _work_experience >=14.0 && _job_description == 6.0{
            println!("{}, Your position is SES",_name_surname);
        }
    }

    if _public_servant_role == 3.0 {
        println!("Enter Your Job Description in your Public Servant Role: ");
        println!("Type 1 for Paralegal");
        println!("Type 2 for Junior Associate");
        println!("Type 3 for Associate");
        println!("Type 4 for Senior Associate 1-2");
        println!("Type 5 for Senior Associate 3-4");
        println!("Type 6 for Partner");
        io::stdin().read_line(&mut input8).expect("Not a valid string");
        let _job_description:f64 = input8.trim().parse().expect("Not a valid number");

        if _work_experience >= 1.0 && _work_experience <=2.0 && _job_description == 1.0 {
            println!("{}, Your position is APS 1-2",_name_surname);
        }
        else {
            println!("You don't hold any position");
        }
        if _work_experience >=3.0 && _work_experience <=5.0 && _job_description == 2.0{
            println!("{}, Your position is APS 3-5",_name_surname );
        }
        if _work_experience >5.0 && _work_experience <8.0 && _job_description == 3.0{
            println!("{}, Your position is APS 5-8",_name_surname);
        }
        if _work_experience >=8.0 && _work_experience <=10.0 && _job_description == 4.0{
            println!("{}, Your position is EL1 8-10",_name_surname);
        }
        if _work_experience >10.0 && _work_experience <=13.0 && _job_description == 5.0{
            println!("{}, Your position is EL2 10-13",_name_surname);
        }
        if _work_experience >=14.0 && _job_description == 6.0{
            println!("{}, Your position is SES",_name_surname);
        }
    }
    if _public_servant_role == 4.0 {
        println!("Enter Your Job Description in your Public Servant Role: ");
        println!("Type 1 for Placement");
        println!("Type 2 for Classroom Teacher");
        println!("Type 3 for Senior Teacher");
        println!("Type 4 for Leading Teacher");
        println!("Type 5 for Deputy Principal");
        println!("Type 6 for Principal");
        io::stdin().read_line(&mut input9).expect("Not a valid string");
        let _job_description:f64 = input9.trim().parse().expect("Not a valid number");

        if _work_experience >= 1.0 && _work_experience <=2.0 && _job_description == 1.0 {
            println!("{}, Your position is APS 1-2",_name_surname);
        }
        else {
            println!("You don't hold any position");
        }
        if _work_experience >=3.0 && _work_experience <=5.0 && _job_description == 2.0{
            println!("{}, Your position is APS 3-5",_name_surname );
        }
        if _work_experience >5.0 && _work_experience <8.0 && _job_description == 3.0{
            println!("{}, Your position is APS 5-8",_name_surname);
        }
        if _work_experience >=8.0 && _work_experience <=10.0 && _job_description == 4.0{
            println!("{}, Your position is EL1 8-10",_name_surname);
        }
        if _work_experience >10.0 && _work_experience <=13.0 && _job_description == 5.0{
            println!("{}, Your position is EL2 10-13",_name_surname);
        }
        if _work_experience >=14.0 && _job_description == 6.0{
            println!("{}, Your position is SES",_name_surname);
        }
    
    }
}
