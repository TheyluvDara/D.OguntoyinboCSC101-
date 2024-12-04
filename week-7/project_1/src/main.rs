
fn main() {
    println!("What calculation do you want to do?");
    let equations = [
        "1 = Area of Trapezium formula",
        "2 = Area of the rhombus formula",
        "3 = Area of Parallelogram formula",
        "4 = Area of Cube formula",
        "5 = Volume of Cylinder formula",
    ];

    for  i in equations.iter() {
        println!("{}", i );
    }

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    match choice {
        1 => area_of_trapezium_formula(),
        2 => area_of_rhombus_formula(),
        3 => area_of_parallelogram_formula(),
        4 => area_of_cube_formula(),
        5 => volume_of_cylinder_formula(),
        _ => println!("Invalid choice"),
    }
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return get_input(prompt);
        }
    }
}

fn area_of_trapezium_formula() {
    println!("Area of Trapezium formula");
    let side1 = get_input("Enter the value of the first side: ");
    let side2 = get_input("Enter the value of the second side: ");
    let distance = get_input("Enter the distance between the parallel sides: ");

    let area = 0.5 * (side1 + side2) * distance;
    println!("The area of the trapezium is: {}", area);
}

fn area_of_rhombus_formula() {
    println!("Area of rhombus formula");
    let d1 = get_input("Enter the value of your first diagonal: ");
    let d2 = get_input("Enter the value of your second diagonal: ");

    let area = (d1 * d2) / 2.0;
    println!("The area of the rhombus is: {}", area);
}

fn area_of_parallelogram_formula() {
    println!("Area of Parallelogram formula");
    let base = get_input("Enter the base of the parallelogram: ");
    let height = get_input("Enter the height of the parallelogram: ");

    let area = base * height;
    println!("The area of the parallelogram is: {}", area);
}

fn area_of_cube_formula() {
    println!("Area of Cube formula");
    let side = get_input("Enter the length of one side of the cube: ");

    let area = 6.0 * (side * side);
    println!("The area of the cube is: {}", area);
}

fn volume_of_cylinder_formula() {
    println!("Volume of Cylinder formula");
    let radius = get_input("Enter the radius of the cylinder: ");
    let height = get_input("Enter the height of the cylinder: ");

    let pi = 3.14;

    let volume = pi * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}