use std::io;

fn main() {
    println!("Choose an occupation type");
    let public_servant = vec!["1 = Office Administrator", "2 = Academic", "3 = Lawyer", "4 = Teacher"];

    for i in 0..public_servant.len() {
        println!("{}",public_servant[i]);
    }

    let mut input1 = String::new();
    println!("Pick between 1-4");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice: i32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1-4");
            return;
        }
    };

    let option = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];
    let index:usize = ((choice - 1)).try_into().unwrap();
    let occupation =  &option[index];
    println!(" You are a/an {}", occupation);


    println!("How many years of work experience do you have: ");
    let mut input_years = String::new();
    io::stdin().read_line(&mut input_years).expect("Failed to read input");
    let years: u32 = match input_years.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    
    match choice {
        1 => office_administrator(years),
        2 => academic(years),
        3 => lawyer(years),
        4 => teacher(years),
        _ => println!("Invalid choice"),
    }
}


fn office_administrator(x: u32)  {
    let office_administrator = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];

    match x {
        1_u32..3_u32 => println!(" Your Position is  a/an {}",office_administrator[0]),
        3_u32..5_u32 => println!("Your Position is  a/an {}",office_administrator[1]),
        5_u32..8_u32 => println!("Your Position is  a/an {}",office_administrator[2]),
        8_u32..10_u32 => println!("Your Position is  a/an {}",office_administrator[3]),
        10_u32..13_u32 => println!("Your Position is  a/an {}",office_administrator[4]),
        _ => println!("Your Position is  a/an {}",office_administrator[5]),
    };
}

fn academic(x: u32)  {
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];

    match x {
        1_u32..3_u32 => println!("Your Position is  a/an {}",academic[0]),
        3_u32..5_u32 => println!("Your Position is  a/an {}",academic[1]),
        5_u32..8_u32 => println!("Your Position is  a/an {}",academic[2]),
        8_u32..10_u32 => println!("Your Position is  a/an {}",academic[3]),
        10_u32..13_u32 => println!("Your Position is  a/an {}",academic[4]),
        _ => println!("Your Position is  a/an {}",academic[5]),
    };
}

fn lawyer(x: u32)  {
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    match x {
        1_u32..3_u32 => println!("Your Position is  a/an {}",lawyer[0]),
        3_u32..5_u32 => println!("Your Position is  a/an {}",lawyer[1]),
        5_u32..8_u32 => println!("Your Position is  a/an {}",lawyer[2]),
        8_u32..10_u32 => println!("Your Position is  a/an {}",lawyer[3]),
        10_u32..13_u32 => println!("Your Position is  a/an {}",lawyer[4]),
        _ => println!("Your Position is  a/an {}",lawyer[5]),
    };
}

fn teacher(x: u32) {
    let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    match x {
        1_u32..3_u32 => println!("Your Position is  a/an {}",teacher[0]),
        3_u32..5_u32 => println!("Your Position is  a/an {}",teacher[1]),
        5_u32..8_u32 => println!("Your Position is  a/an {}",teacher[2]),
        8_u32..10_u32 => println!("Your Position is  a/an {}",teacher[3]),
        10_u32..13_u32 => println!("Your Position is  a/an {}",teacher[4]),
        _ => println!("Your Position is  a/an {}",teacher[5]),
    };
}