fn main() {
    //println!("Hello, world!");

    let x = vex![100, 200, 300];
    borrow+vector(&x); // passing reference

    println!("Printing the value from main() x[0]={}", x[0]);
    println!("****************************");

}

borrow_vector(z:&Vec<i32>){
    println!("**********************");
    println!("Inside print_vector function {:?} \n",z);
    println!("------------------------");
}