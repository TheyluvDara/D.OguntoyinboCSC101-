use std::io;

fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();

    println!("How many people are you entering into the program?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Error reading input");
    let people: i32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    for i in 0..people{
        println!("Enter the name of {} person:", i+1);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Error reading input");
        let people_names: String = input2.trim().parse().expect("Invalid input");
        names.push(people_names);

        println!("How many years of experience in programming do you have:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Error reading input");
        let people_years: u32 = match input3.trim().parse() {
            Ok(years) => years,
            Err(_) => {
                println!("Please enter a valid number of years");
                return;
            }
        };
        years.push(people_years);
    }

    let (max , index) = highest_value(years);

    println!("{} has the most experience with {} years of experience", names[index], max);
     
    
}

fn highest_value (years: Vec<u32>) -> (u32, usize) {
    let mut max = years[0];
    let mut index = 0;
    for i in 0..years.len() {
        if years[i] > max {
            max = years[i];
            index = i;
        }
    };

    (max, index)
}
