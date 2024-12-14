use std::io::Write;
use std::io::Read;

fn main() {
    
    let name_of_commisioner = vec![
        "Aigbogun Alamba Dauda".to_string(), 
        "Murtala Afeez Bendu".to_string(), 
        "Okorocha Calistus Ogbona".to_string(), 
        "Adewale Jimoh Akanbi".to_string(), 
        "Osazuwa Faith Etieye".to_string(),
    ];


    let ministry = vec![
        "Internal Affairs".to_string(),
        "Justices".to_string(),
        "Defence".to_string(),
        "Power & Street ".to_string(),
        "Petroleum".to_string(),
    ];


    let geopolitical_zone = vec![
        "South West".to_string(),
        "North East".to_string(),
        "South South".to_string(),
        "South West".to_string(), 
        "South East".to_string(),
    ];

    let mut file = std::fs::File::create("List_of_convicted_ministers.txt").expect("create failed");

    file.write_all(b"S/N  | NAME OF COMMISIONER        | MINISTRY          | GEOPOLITICAL ZONE\n").expect("write failed");
    file.write_all(b"-------------------------------------------------------------------------\n").expect("write failed");

    for i in 0..name_of_commisioner.len() {
        let l1 = name_of_commisioner.get(i).unwrap();
        let l2 = ministry.get(i).unwrap();
        let l3 = geopolitical_zone.get(i).unwrap();

        let line = format!("{:<4} | {:<26} | {:<17} | {:<17}\n" , i + 1, l1, l2, l3);

        file.write_all(line.as_bytes()).expect("write failed");
    };

    println!("Table of Convicted Ministers from different geopolitical zones: ");
    println!("");

    let mut file_2 = std::fs::File::open("List_of_convicted_ministers.txt").unwrap();
    let mut content = String::new();
    file_2.read_to_string(&mut content).unwrap();
    print!("{}", content);

}
