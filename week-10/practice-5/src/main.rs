// NOTE: FROM PRACTICE 1-4, IT GIVES ERRORS, BUT FROM PRACTICE 5-9, IT DOESN'T GIVE ERRORS

fn main() {

    // a list of numbers
    let x = vec![100,200,300];
    borrow_vector(&x); // passing reference

    println!("Printing the values from main() x[0]={}", x[0]);
    println!("*****************************");
}

fn borrow_vector(z:&Vec<i32>){

    println!("*****************************");
    println!("Inside print_vector function {:?} \n",z);
    println!("-----------------------------");
}
