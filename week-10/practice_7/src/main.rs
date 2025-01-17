struct Emplyee {
    name: String,
    comapany:String,
    age:u32

}


fn main() {
    //println!("Hello, world!");
    let emp1 = Employee {
        comapny:String::from("Enrst & Young"),
        name:String::from("Ebiibiong Jessica"),
        age:25
    };
    println!("Name = {}",emp.name);
    println!("Coampnay = {}", emp1.comapnay);
    println!("Age = {}", emp1.age);
}
