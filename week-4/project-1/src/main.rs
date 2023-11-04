    use std::io;

fn main() {
    
    let mut distance = String::new();
    println!("Enter Distance");
    io::stdin().read_line(&mut distance).expect("Not a valid string");
    let d:f32 = distance.trim().parse().expect("Not a valid number");


    let mut time = String::new();
    println!("Enter Time");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let t:f32 = time.trim().parse().expect("Not a valid number");


    let y = d * 1.609;    // 1mile is 1.609km
    let s = y / t;
    println!("speed is {} km/h",s); 
}
