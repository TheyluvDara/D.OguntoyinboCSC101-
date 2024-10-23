fn main(){
    let p:f64 = 510_000.00;
    let r:f64 = 5.0;
    let n:f64 = 3.0;

    let a = p * (1.0 - (r/100.0)).powf(n);
    let rounded = a.round();
    println!("The new price is: {}", a);
    println!("{} rounded is {}", a, rounded)
}