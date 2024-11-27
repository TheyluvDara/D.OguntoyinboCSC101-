fn main() {
    println!("What calculation do you want to do?");
    let eqaution = ["1 = Area of Trapezium formula", "2 = Area of the rhombus formula","3 = Area of Parallelogram formula", "4 = Area of Cube formula ", "5 = Volume of Cylinder formula " ];

    for n in eqaution.iter() {
        println!("{}", n);
    }

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    
    if choice == 1 {
        area_of_trapezium_formula();
    } else if choice == 2 {
        area_of_rhombus_formula();
    } else if choice == 3 {
        area_of_parallelogram_formula();
    } else if choice == 4 {
        area_of_cube_formula();
    } else if choice == 5 {
        volume_of_cylinder_formula();
    } else {
        println!("Invalid choice");
    }
}

fn area_of_trapezium_formula() {
    println!("Area of trappezium ");
    println!("Enter the value of the first side: ");
    let mut side1 = String::new();
    std::io::stdin().read_line(&mut side1).expect("Failed to read input");
    let side1: f64 = match side1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the value of the second side: ");
    let mut side2 = String::new();
    std::io::stdin().read_line(&mut side2).expect("Failed to read input");
    let side2: f64 = match side2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the distance between the parallel sides: ");
    let mut distance = String::new();
    std::io::stdin().read_line(&mut distance).expect("Failed to read input");
    let distance: f64 = match distance.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let area = 0.5 * (side1 + side2) * distance;
    println!("The area of the trapezium is: {}", area);
}

fn area_of_rhombus_formula() {
    println!("Area of rhombus ");
    println!("Enter the value of your first diagonal: ");
    let mut d1 = String::new();
    std::io::stdin().read_line(&mut d1).expect("Failed to read input");
    let d1: f64 = match d1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the value of your second diagonal: ");
    let mut d2 = String::new();
    std::io::stdin().read_line(&mut d2).expect("Failed to read input");
    let d2: f64 = match d2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let area = (d1 * d2) / 2.0;
    println!("The area of the rhombus is: {}", area);

}

fn area_of_parallelogram_formula() {
    println!("Area of parallelogram ");
    println!("Enter the base of the parallelogram: ");
    let mut base = String::new();
    std::io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f64 = match base.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the height of the parallelogram: ");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let area = base * height;
    println!("The area of the parallelogram is: {}", area);
}

fn area_of_cube_formula() {
    println!("Area of cube ");
    println!("Enter the length of one side of the cube: ");
    let mut side = String::new();
    std::io::stdin().read_line(&mut side).expect("Failed to read input");
    let side: f64 = match side.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let area = 6.0 * (side * side);
    println!("The area of the cube is: {}", area);
}

fn volume_of_cylinder_formula() {
    println!("Volume of cylinder ");
    println!("Enter the radius of the cylinder: ");
    let mut radius = String::new();
    std::io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the height of the cylinder: ");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let pi = 3.14;

    let volume = pi * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}