fn main(){
    //varaible p represents principle
    let p:i32 = 52_000_000;
    println!("principal is : {}", p);

    // variable r represent rate
    let r:i32 = 10;
    println!("rate is : {}", r);

    //variable t represent time
    let t:i32 = 5;
    println!("time is : {}", t);

    // this block of code prints a
    let a = p * (1 + (r / 100)) * t;
    println!("a is : {}",a);

    // this line of code is to fine the compound intrest
    let ci = a - p;
    println!("the compound intrest is: {}", ci);



}