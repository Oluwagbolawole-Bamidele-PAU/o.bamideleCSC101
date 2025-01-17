use std::io;

//Method to print the get value
fn value (n:Option<&char>)
{
    println!("Element of vector {:?}", n);
}

fn main() {
    //println!("Hello, world!");
    let v = vec!['R', 'U', 'S', 'T', 'A', 'I', 'A', 'N'];

    let mut inut1 = String::new();
    println!("/nEnter an index value btw (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    //index is the non negative nalue which is smaller than the size of the vector
    let index:usize = input.trim().parse().expect("invalid input");

    // getting value at given index value
    let ch: Option<&char> = v.get (index);
    value(ch);
}
