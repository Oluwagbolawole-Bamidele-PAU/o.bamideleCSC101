fn main() {
    //println!("Hello, world!");
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    //Age vector
    let age = vec![16, 17, 19, 20, 21, 18, 23];

    print!("/nAge allocation:/n");

    //loop to interate elements in vector
    for i in 0..age.len()
    {
        //interating through i on the vector
        print!("{} is {} year old/n", name[i], age[i]);
    }
}
