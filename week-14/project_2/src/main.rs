use std::io;
use std::io::Read;

fn main() {
    println!("Database access application");
    let positions = ["An Administrator", "A Project Manager", "An Employee","A Customer","A Vendor"];
    println!("Pick an option");
    for i in 0..positions.len() {
        println!("{} - {}", i + 1, positions[i]);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let position: usize = input.trim().parse().expect("Invalid input");

    if position == 1 {
        println!("You are an Administrator");
        admin_operations();
    } else if position == 2 {
        println!("You are a Project Manager");
        project_manager_operations();
    } else if position == 3 {
        println!("You are an Employee");
        employee_operations();
    } else if position == 4 {
        println!("You are a Customer");
        customer_operations();
    } else if position == 5 {
        println!("You are a Vendor");
        vendor_operations();
    }
}

fn admin_operations() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn project_manager_operations() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn employee_operations() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer_operations() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vendor_operations() {
    let mut file = std::fs::File::open("data-plan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}