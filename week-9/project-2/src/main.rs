use std::io::Read;
use std::io::Write;

fn main() {
    let student_names = [
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];
    let matric_numbers = [
        "ACC10211111",
        "ECO10101101",
        "CSC10328828",
        "EEE1020202",
        "MEE10202001",
    ];
    let departments = [
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];
    let levels = [100, 100, 300, 200, 100];

    let mut students: Vec<(&str, &str, &str, i32)> = Vec::new();
    for i in 0..student_names.len() {
        students.push((student_names[i], matric_numbers[i], departments[i], levels[i]));
    }

    let mut file = std::fs::File::create("PAU SMIS.txt").expect("create failed");

    file.write_all(b"                        PAU SMIS                        \n").expect("write failed");
    file.write_all(b"Student Name    | Matric. Number | Department   | Level\n").expect("write failed");
    file.write_all(b"-------------------------------------------------------\n").expect("write failed");

    for student in &students {

        let line = format!("{:<15} | {:<14} | {:<12} | {:<3}\n", student.0, student.1, student.2, student.3);
        file.write_all(line.as_bytes()).expect("write failed");
    
    }

    let mut file_2 = std::fs::File::open("PAU SMIS.txt").unwrap();
    let mut content = String::new();
    file_2.read_to_string(&mut content).unwrap();
    print!("{}", content);



    println!("File created successfully")
}
