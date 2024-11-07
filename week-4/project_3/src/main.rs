use std::io;

fn main() {
    println!("\n             Main                          Price ");
    println!("\n  P = Poundo Yam/Edinkaiko Soup          - N3,200");
    println!("\n  F = Fried Rice & Chicken               - N3,000");
    println!("\n  A = Amala & Ewedu Soup                 - N2,500");
    println!("\n  E = Eba & Egusi Soup                   - N2,000");
    println!("\n  W = White Rice & Stew                  - N2,500");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What type of food do you want: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food =  input1.trim().to_uppercase();

    println!("Enter the quantity you want: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("not a valid number");

    if food == "P"{
        let  price:f32 = 3200.0 * quantity;
        if price > 10000.0 {
           let final_price:f32 = price - (price * 5.0 )/ 100.0;
            println!("Your discounted total is: {}", final_price);
        } else {
            println!("Your total is: {}", price);
        }
    } else if food == "F" {
        let  price:f32 = 3000.0 * quantity;
        if price > 10000.0 {
           let final_price:f32 = price - (price * 5.0 )/ 100.0;
            println!("Your discounted total is: {}", final_price);
        } else {
            println!("Your total is: {}", price);
        }
    } else if food == "A" {
        let  price:f32 = 2500.0 * quantity;
        if price > 10000.0 {
           let final_price:f32 = price - (price * 5.0 )/ 100.0;
            println!("Your discounted total is: {}", final_price);
        } else {
            println!("Your total is: {}", price);
        }
    } else if food == "E" {
        let  price:f32 = 2000.0 * quantity;
        if price > 10000.0 {
           let final_price:f32 = price - (price * 5.0 )/ 100.0;
            println!("Your discounted total is: {}", final_price);
        } else {
            println!("Your total is: {}", price);
        }
    } else if food == "W" {
        let  price:f32 = 2500.0 * quantity;
        if price > 10000.0 {
           let final_price:f32 = price - (price * 5.0 )/ 100.0;
            println!("Your discounted total is: {}", final_price);
        } else {
            println!("Your total is: {}", price);
        }
    } else {
        println!("Invalid food item")
    }
}
