use std::io;

fn main(){
    let mut input1 = String::new();
    println!("Good day, How many sibling do you have ");
    io::stdin().read_line(&mut input1).expect("Wrong input was entered");
    let mut number_of_siblings:u32 = input1.trim().parse().expect("wrong input was enterd");
    
   /*  if number_of_siblings == 1 {
            let mut input2 = String::new();
            let mut input3 = String::new();
            let mut input4 = String::new();
            let mut input5 = String::new(); 

            println!("Now that I know have many, I have some question ?");
            println!("Lets start with the first sibling");
            println!("What is their name?");
            io::stdin().read_line(&mut input2).expect("Wrong input was entered");
            println!("What is their age?");
            io::stdin().read_line(&mut input3).expect("Wrong input was entered");
            println!("What is their gender?");
            io::stdin().read_line(&mut input4).expect("Wrong input was entered");
            println!("What is their country of residence?");
            io::stdin().read_line(&mut input5).expect("Wrong input was entered");
    }

    if number_of_siblings == 2 {
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new(); 

        let mut input6 = String::new();
        let mut input7 = String::new();
        let mut input8 = String::new();
        let mut input9 = String::new(); 

        println!("Now that I know have many, I have some question ?");
        println!("Lets start with the first sibling");
        println!("What is their name?");
        io::stdin().read_line(&mut input2).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input3).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input4).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input5).expect("Wrong input was entered");

        println!("Lets start with the second sibling");
        println!("What is their name?");
        io::stdin().read_line(&mut input6).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input7).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input8).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input9).expect("Wrong input was entered");
    }

    if number_of_siblings == 3 {
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new(); 

        let mut input6 = String::new();
        let mut input7 = String::new();
        let mut input8 = String::new();
        let mut input9 = String::new(); 

        let mut input10 = String::new();
        let mut input11 = String::new();
        let mut input12 = String::new();
        let mut input13 = String::new(); 

        println!("Now that I know have many, I have some question ?");
        println!("Lets start with the first sibling");
        println!("What is their name?");
        io::stdin().read_line(&mut input2).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input3).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input4).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input5).expect("Wrong input was entered");

        
        println!("Lets start with the second sibling");
        println!("What is their name?");
        io::stdin().read_line(&mut input6).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input7).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input8).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input9).expect("Wrong input was entered");

        println!("Lets start with the third sibling");
        println!("What is their name?");
        io::stdin().read_line(&mut input10).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input11).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input12).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input13).expect("Wrong input was entered");
    }

    if number_of_siblings >= 4 {
        println!("Sorry, number of sibling exceeds the number of avaible for registration.");
        println!("Try withing ranges 1 - 3");
    }*/

    for i in 1 ..= number_of_siblings {
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new(); 

        io::stdin().read_line(&mut input3).expect("Wrong input was entered");
        let mut age:u32 = input3.trim().parse().expect("wrong input was enterd");

        println!("Now that I know have many, I have some question ?");
        println!("Lets start with the sibling info");
        println!("What is their name?");
        io::stdin().read_line(&mut input2).expect("Wrong input was entered");
        println!("What is their age?");
        io::stdin().read_line(&mut input3).expect("Wrong input was entered");
        println!("What is their gender?");
        io::stdin().read_line(&mut input4).expect("Wrong input was entered");
        println!("What is their country of residence?");
        io::stdin().read_line(&mut input5).expect("Wrong input was entered");
        println!("So the name of your sibling is {}, They are {} old,There gender is {}, and they live in {}", input2, input3, input4, input5);

        if age >= 18 {
            let mut input6 = String::new();
            println!("I see that you are 18, are you married, single or in a relationship");
            println!("1 - married
                    2 - single
                    3 - in a relationship    
                ");
            io::stdin().read_line(&mut input6).expect("Wrong input was entered");
            let input6_1:u32 = input6.trim().parse().expect("Wrong input");
            
            if input6_1 == 1 {
                println!("You chosen married
                    How many children do you have
                ");
                
                let mut input7 = String::new();
                io::stdin().read_line(&mut input7).expect("Wrong input was entered");
                let mut no_children:u32 = input7.trim().parse().expect("wrong input was enterd");

                for i in 1 .. no_children {

                    let mut input8 = String::new();
                    let mut input9 = String::new();
                    let mut input10 = String::new();
                    //let mut input11 = String::new(); 

                    println!("I see you have {}, children", no_children);
                    println!("Let's move on to childrens info");
                    println!("What is their name?");
                    io::stdin().read_line(&mut input8).expect("Wrong input was entered");
                    println!("What is their age?");
                    io::stdin().read_line(&mut input9).expect("Wrong input was entered");
                    println!("What is their school?");
                    io::stdin().read_line(&mut input10).expect("Wrong input was entered");
                   // println!("What is their country of residence?");
                    //io::stdin().read_line(&mut input11).expect("Wrong input was entered");
                    println!("The name of the kid is {}, there age is {}, and they school at {}, they live in{} ", input8, input9, input10, input5);




                }
            } else if input6_1 == 2
            {

                   println!("I see that you are single, I'm sorry to hear that");
                   println!(".
                   Are you a studunt(1) or are you employed(2) ");
                   let mut input11 = String::new();
                   io::stdin().read_line(&mut input11).expect("wrong input");
                   let input11_1:u32 = input11.trim().parse().expect("Wrong input");

                   if input11_1 == 1 {
                    let mut university = String::new();
                    let mut course_name = String::new();
                    let mut year_of_study = String::new();

                    println!("What is your university name name");
                    io::stdin().read_line(&mut university).expect("wrong input");
                    println!("What is your course name");
                    io::stdin().read_line(&mut course_name).expect("wrong input");
                    println!("What is your year of study");
                    io::stdin().read_line(&mut year_of_study).expect("wrong input");

                    println!("Are you studying at home(1) or abroad(2)");
                    let mut input11_1_2 = String::new();
                    io::stdin().read_line(&mut input11_1_2).expect("wrong input");
                    let study_location:u32 = input11_1_2.trim().parse().expect("Wrong input");
                    
                    if study_location == 1{
                        println!("You are studying at home which is {}", input5);
                        println!("YOu attent {}, you study{} and your study of year is {}, your uni is located in {}", university, course_name, year_of_study, input5)
                    } else {
                        println!("YOu are studying abround, where abroud");
                        let mut input11_1_2_3 = String::new();
                        io::stdin().read_line(&mut input11_1_2_3).expect("wrong input");
                        let study_location_1:u32 = input11_1_2_3.trim().parse().expect("Wrong input");
                        println!("YOu attent {}, you study{} and your study of year is {}, your uni is located in {}", university, course_name, year_of_study, input11_1_2_3)
                    }

                   }else {
                    println!("That means that you are employed, Congrats");
                    println!("is the job on-site(0), remote(1), hybrid(2)");

                    let mut job_type = String::new();
                    io::stdin().read_line(&mut job_type).expect("wrong input");
                    let input12:u32 = job_type.trim().parse().expect("Wrong input");
                    if input12 == 1{

                        let mut company_name = String::new();
                        let mut job_title = String::new();
                        let mut industry_sector = String::new();

                        println!("What is your comapany name");
                        io::stdin().read_line(&mut company_name).expect("wrong input");
                        println!("What is your job title");
                        io::stdin().read_line(&mut job_title).expect("wrong input");
                        println!("What is your What is your industry sector");
                        io::stdin().read_line(&mut industry_sector).expect("wrong input");

                        println!("Company name is {}, job title is {}, industry_sector{}", company_name, job_title, industry_sector);


                    }
                    
                   }
                
             }else if input6_1 == 3 {

                println!("That means you are in a relation, congracts to your significant other, you seem like a nice person");
                let mut babes_name = String::new();
                let mut how_long_relationship = String::new();

                println!("What is your babe name");
                io::stdin().read_line(&mut babes_name).expect("wrong input");
                println!("HOw long is your relationship been on for");
                io::stdin().read_line(&mut how_long_relationship).expect("wrong input");
                let how_long_relationship_int:u32 = how_long_relationship.trim().parse().expect("Wrong input");

                let mut sleeping_together = String::new();
                println!("Do you live with your babe? Yes(1) NO(0)");
                io::stdin().read_line(&mut sleeping_together).expect("wrong input");
                let sleeping_together_int:u32 = sleeping_together.trim().parse().expect("Wrong input");

                if sleeping_together_int == 1 {
                    let mut babe_location = String::new();
                    println!("Where does babe live?");
                    io::stdin().read_line(&mut babe_location).expect("wrong input");
                    println!("That means that your babe name is {}, you've been together for {}, and you live at {}", babes_name, how_long_relationship_int, babe_location);
                } else {
                    println!("That means that your babe name is {}, you've been together for {}, and you live at {}", babes_name, how_long_relationship_int, input5);
                }


             }
             

        } if age < 18 {
            println!("Oh, intresting I see that you are under 18 I have some questions for you. Are you done with WAEC (0) ")
        }
        
    }

}