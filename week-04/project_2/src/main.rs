use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Hello and welcome, my pupose is to inform all employees about there insintive based on there qualifications!");
    println!("before we start I need to know do you have any Expeirence");
    println!("press 1 for yes");
    println!("press 2 for no");
    io::stdin().read_line(&mut input1).expect("Wrong input");
    let exp:i32 = input1.trim().parse().expect("Wrong input");


    if exp == 1 {
        println!("Perfect, that mean that you are elegible for our age package!");
        println!("Our age packages is as followed :
                40 and above: N 1,560,000
                30 - 39: N 1,480,00
                28 and bellow: N 1,300,00
            
            ");
        println!(" Let's get start first I'm going to need your age :");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let age:u32 = input2.trim().parse().expect("Wrong input");

        if age >= 40 {
            println!("Congracts your insentive for this year is: N 1,560,000");
        } else if age >= 30 && age < 40{
            println!("Congracts your insentive for this year is: N 1,480,000");
        }else if age <= 28 {
            println!("Congracts your insentive for this year is: N 1,300,000");
        }
    } else {
        println!("sorry you are not elegible for out age package but you will recieve : N100,000");

    }
    
}
