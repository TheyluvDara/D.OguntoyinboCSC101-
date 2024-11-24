use std::io;
fn main() {
    println!("Calculate Your Annual incentive")
    loop {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Are you experienced or inexperienced : ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let experience = input1.trim().to_uppercase();

        if experience == "experienced" {
            println!("Enter your age: ");
            io::stdin().read_line(&mut input2).expect("Not a valid string");
            let age:i32 = input2.trim().parse().expect("not a valid number");

            if age >= 40 {
                println!("Annual Incentive: N1,560,000");
                break;
            } else if age >= 30 && age <= 40 {
                println!("Annual Incentive: N1,480,000");
                break;
            }else if age >=18 && age < 30 {
                println!("Annual Incentive: N1,300,000");
                break;
            } else {
                println!("Abnormal age");
                break;
            }

        } else {
            println!("Annual Incentive: N100,000");
            break;
        }

    }
}
