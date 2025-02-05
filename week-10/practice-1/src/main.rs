struct Laptop{
    model: String,
    price: u32,
    amount: u32
}

impl Laptop{
    fn sell(&mut self, qty: u32)
    {
        self.amount -= qty; 
    }

    fn addToInventory(&mut self, qty: u32)
    {
        self.amount += qty; 
    }
}

fn main()
{
    let mut dells = Laptop{model:String::from("DELL"), price: 850000, amount: 4};
    let mut hps = Laptop{model:String::from("HP"), price: 650000, amount: 10};
    let mut ibms = Laptop{model:String::from("IBM"), price: 755000, amount: 6};
    let mut toshibas = Laptop{model:String::from("TOSHIBA"), price: 550000, amount: 10};
    println!("\n\n      ------  Mr. OGBEIFUNA'S INVENTORY :  ------");  
    displayStruct(&dells);
    displayStruct(&hps);
    displayStruct(&ibms);
    displayStruct(&toshibas);

    let costForThreeAll = calCost(&mut dells, &mut hps, &mut ibms, &mut toshibas, 3,3,3,3);
    
    println!("Cost for customer to buy 3 Dells, 3 HPs, 3 Toshibas and 3 IBMs is {} ***TRANSACTION COMPLETED***", costForThreeAll);

}   

fn displayStruct(lapt: &Laptop)
{
    println!("MODEL: {},\nPRICE: {},\nAMOUNT AVAILABLE: {}\n", lapt.model, lapt.price, lapt.amount);
}

fn calCost(laptDell: & mut Laptop,laptHp: &mut Laptop,laptIbm: &mut Laptop,laptTosh: &mut Laptop,dellAmt: u32, hpAmt: u32, ibmAmt: u32, toshAmt: u32) -> u32
{
    let mut cost: u32;
    if laptDell.amount >= dellAmt && laptHp.amount >= hpAmt && laptIbm.amount >= ibmAmt && laptTosh.amount >= toshAmt
    {
        laptDell.sell(dellAmt);
        laptHp.sell(hpAmt);
        laptIbm.sell(ibmAmt);
        lapTosh.sell(toshAmt);
        cost = (laptDell.price * dellAmt) + (laptHp.price * hpAmt) + (laptTosh.price * toshAmt) + (laptIbm.price * ibmAmt);
    }
    else
    {
        println!("INSUFFICIENT AMOUNT IN INVENTORY FOR SALE");
        cost = 0;
    }

    return cost;
    
}