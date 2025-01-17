struct Rectangle {
    width:u32, height:u32
}

//logic to calculate area of a rectangle

imp Rctangle {
    fn area(&self)->u32 {
        //use ethe operator to fetch the value of a field via the self keyword
        self.width* slef.height
    }
}
fn main() {
    //println!("Hello, world!");
    // instanatiate the structure
    let small = Rectangle {
        width:10
        height:20
    };
    //print the rectangle's are
    println!("width is {} height is {} area of Rectangle is {}",small.width,small.height,small.are());

}
