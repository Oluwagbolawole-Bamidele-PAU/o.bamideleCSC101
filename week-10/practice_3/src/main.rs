fn main() {
    //println!("Hello, world!");
    let v= vec![20, 40, 60, 80];
    // vector v owns the object in heap

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}",v);

}

fn display(v:Vec<i32>)->Vec<i32> {
    // retunring smae vector
    println!("inside display {:?}",v);
    return v;
    
}