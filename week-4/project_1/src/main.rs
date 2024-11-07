use std::io;

fn main() {
    println!("Finding the roots of quadratic equations")

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("not a valid number");

    let discriminant = b.powi(2) - 4.0 * a * c;
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

    if discriminant > 0.0 {
        println!("The roots are real and distinct: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        println!("The roots are real and equal: {}", root1);
    } else {
        println!("The roots are not real: {} and {}", root1, root2);
    }
}
