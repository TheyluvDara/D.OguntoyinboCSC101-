use std::io::Read;
use std::io;
use std::io::Write;

fn main() {

    let mut student_names = Vec::new();
    let mut matric_numbers = Vec::new();
    let mut departments = Vec::new();
    let mut levels = Vec::new();

    println!("How many students information are you inputing");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num_students: u32 = input1.trim().parse().expect("Please type a number");

    for i in 0..num_students {
        println!("Enter the student name:(i.e Oluchi Mordi) ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let student_name = input2.trim().to_string();
        

        student_names.push(student_name);

        println!();

        println!("Enter your matric nymber: (e.g ACC10211111)");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read line");
        let matric_number = input3.trim().to_string();
        if matric_number.len() != 11 {
            println!("Please enter a valid matric number");
            return;
        }

        matric_numbers.push(matric_number);

        println!();

        println!("Enter your department: ");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read line");
        let department = input4.trim().to_string();

        departments.push(department);

        println!();

        println!("Enter your level: ");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read line");
        let level: i32 = match input5.trim().parse() {
            Ok(num) => {
                if num < 100 || num > 800 {
                    println!("Please enter a valid level");
                    return;
                }
                num
            },
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };
        levels.push(level);

        println!();

    }


    

    let mut students: Vec<(String, String, String, i32)> = Vec::new();
    for i in 0..student_names.len() {
        students.push((student_names[i].clone(), matric_numbers[i].clone(), departments[i].clone(), levels[i]));
    }

    let mut file = std::fs::File::create("PAU SMIS.txt").expect("create failed");

    file.write_all(b"                               PAU SMIS                             \n").expect("write failed");
    file.write_all(b"     Student Name    | Matric. Number |     Department       | Level\n").expect("write failed");
    file.write_all(b"--------------------------------------------------------------------\n").expect("write failed");

    for student in &students {

        let line = format!("{:<18} | {:<14} | {:<20} | {:<3}\n", student.0, student.1, student.2, student.3);
        file.write_all(line.as_bytes()).expect("write failed");
    
    }

    let mut file_2 = std::fs::File::open("PAU SMIS.txt").unwrap();
    let mut content = String::new();
    file_2.read_to_string(&mut content).unwrap();
    print!("{}", content);



    println!("File created successfully")
}
