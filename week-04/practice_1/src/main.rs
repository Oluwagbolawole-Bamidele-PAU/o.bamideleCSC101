use std::io;

fn main() {
    println!("Student Information Management System!");

    //input name
    println!("\nPlease Enter your name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");
    println!("your name is: {}", name);

    //input age
    println!("Please Enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    println!("your age is: {}", age);


}
