struct Emplyee {
    ceo:String,
    comapny:String,
    age:u32
}


fn main() {
    //println!("Hello, world!");
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc"),
        ceo:String::from("Sundia Pichai"),
        age:51

    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);

}
// fetch values of specific strustures feilds using the
// operator and print it to the console
fn display(emp:Employee){
    println!("Name is :{} comapny is {} age is {}", emp.ceo, emp.company,emp.age);
}
