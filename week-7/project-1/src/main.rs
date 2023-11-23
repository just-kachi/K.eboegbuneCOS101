use std::io;

fn select() {
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

    println!("List of Formula's available");
    println!("Formula for Area of Trapezium = height/2 * (base1 + base2)");
    println!("Formula for Area of a Rhombus = 1/2 * diagonal1 * diagonal2");
    println!("Formula for Area of Parallelogram = base * altitude)");
    println!("Formula for Area of Cube = 6 * (length of the side)^2");
    println!("Formula for Volume of Cylinder = pi * radius^2 * height");

    println!("To solve for Area of Trapezium, Type 1");
    println!("To solve for Area of a Rhombus, Type 2");
    println!("To solve for Area of Parallelogram, Type 3");
    println!("To solve for Area of Cube, Type 4");
    println!("To solve for Volume of Cylinder, Type 5");
    io::stdin().read_line(&mut input11).expect("Not a valid string");
    let _to_solve:f64 = input11.trim().parse().expect("Not a valid number");

    if _to_solve == 1.0 {
    println!("Formula for Area of Trapezium = height/2 * (base1 + base2)");
    println!("Enter your value for the Height: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _height:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value for the First Base: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _base_1:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your value for the Second Base: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _base_2:f64 = input3.trim().parse().expect("Not a valid number");
    
    let t = _height/2.0 * (_base_1 + _base_2);
    println!("The answer is {} cm^2", t)
    }


    if _to_solve == 2.0 {
    println!("Formula for Area of the Rhombus = 1/2 * diagonal1 * diagonal2");
    println!("Enter your value for the First Diagonal: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let _diagonal_1:f64 = input4.trim().parse().expect("Not a valid number");

    println!("Enter your value for the Second Diagonal: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _diagonal_2:f64 = input5.trim().parse().expect("Not a valid number");
    
    let r = 1.0/2.0 * _diagonal_1 * _diagonal_2;
    println!("The answer is {} cm^2", r)
    }

    if _to_solve == 3.0 {
    println!("Formula for Area of Parallelogram = base * altitude)");
    println!("Enter your value for the base: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let _base:f64 = input6.trim().parse().expect("Not a valid number");

    println!("Enter your value for the altitude: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let _altitude:f64 = input7.trim().parse().expect("Not a valid number");

    let p = _base * _altitude;    
    println!("The answer is {} cm^2", p)
    }

    if _to_solve == 4.0 {
    println!("Formula for Area of Cube = 6 * (length of the side)^2");
    println!("Enter your value for the Length of the Side: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let _length_of_the_side:f64 = input8.trim().parse().expect("Not a valid number");

    let x = f64::powf(_length_of_the_side,2.0);
    
    let c = 6.0 * x;
    println!("The answer is {} cm^2", c)
    }

    if _to_solve == 5.0 {
    println!("Formula for Volume of Cylinder = pi * radius^2 * height");
    println!("Enter your value for the radius: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let _radius:f64 = input9.trim().parse().expect("Not a valid number");

    println!("Enter your value for the Height: ");
    io::stdin().read_line(&mut input10).expect("Not a valid string");
    let _height_1:f64 = input10.trim().parse().expect("Not a valid number");   

    let a = 22.0;
    let b = 7.0;
    let pi = a/b;
    let y = f64::powf(_radius,2.0);
    
    let cy = pi * y * _height_1;
    println!("The answer is {} cm^3", cy)
    }
}

fn main(){

    println!("this program allows you to select which formula you want to use");
    select()

}