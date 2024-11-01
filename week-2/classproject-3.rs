fn main(){
    let tv_price: i32 = 510_000;
    let rate_depreciation: i32 = 5;
    let years_after_use: i32 = 3;

    let a = tv_price * (1 - (rate_depreciation / 100) ) ^ years_after_use;
    println!("The value of the TV after 3 years of use is ({})", a);

}