fn main() {
    //println!("Hello, world!");
    let v = vec![10, 20, 30];
    // vecotr v owns the object in heap

    let v2 = v;
    //moves ownership to v2

    diesplay(v2);
    //v2 is moved to display and v2 is invalidated

}

fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
}
