fn main(){

    // This block of code is to calcualate the total number cost of toshiba laptops
    /*
        let toshiba-quantity = 2;
    let toshiba-amount = 450_000;
    let toshiba-price = toshiba-quantity * toshiba-amount;
    println!("this is the total price of Toshiba ") */
    //this is for the toshiba array containing both product Qty and Amount
    let t_a: [i32; 2] = [2, 450_000];
    //Toshiba's total amount
    let t = t_a[0] * t_a[1];
    println!("Toshiba's total amount is {}", t);

    //this the mac array containing both product Qty and Amount
    let m_a: [i32; 2] = [1, 1_500_000];
    //Mac's total amount
    let m = m_a[0] * m_a[1];
    println!("Mac's total amount is {}", m);

    //this is the hp array containing both product Qty and Amount
    let hp_a: [i32; 2] = [3, 750_000];
    //hp's total amount
    let hp = hp_a[0] * hp_a[1];
    println!("HP's total amount is {}", hp);

    //this is the Dell array containing both product Qty and Amount
    let dell_a: [i32; 2] = [3, 2_850_000];
    //Dell's total amount
    let dell = dell_a[0] * dell_a[1];
    println!("Dell's total amount is {}", dell);
    
    //This is the Acer array containing both product Qty and Amount
    let acer_a: [i32; 2] = [1, 250_000];
    let acer = acer_a[0] * acer_a [1];
    println!("Acer's total amount is {}", acer);

    //Total amount
    let total_amount = t + m + hp + dell + acer;
    println!("Total amount for all the items {}", total_amount);

    //Average
    let _average = (total_amount) / (10);


}