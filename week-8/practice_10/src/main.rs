fn main() {
    let b:(i32, bool, f64) = (30, true , 4.9);
    print(b);
}

fn print(x:(i32, bool, f64)) {

    println!("Inside print method");
    let (age,ismale,cgpa) = x ;
    println!("Age is {}, isMale? {}, cgpa is {}", age,ismale,cgpa);
}

