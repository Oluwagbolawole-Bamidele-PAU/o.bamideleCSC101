use std::fs;

fn main() {
    //println!("Hello, world!");
    //Delete a File
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
