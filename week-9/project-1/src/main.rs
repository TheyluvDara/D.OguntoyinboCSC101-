use std::io::Write;



fn main() {
    
    let lager = vec![
        "33 Export".to_string(),
        "Desperados".to_string(),
        "Goldberg".to_string(),
        "Gulder".to_string(),
        "Heineken".to_string(),
        "Star".to_string(),
    ];

    let stout = vec![
        "Legend".to_string(),
        "Turbo King".to_string(),
        "Williams".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    let non_alcoholic = vec![
        "Maltina".to_string(),
        "Amstel Malta".to_string(),
        "Malta Gold".to_string(),
        "Fayrouz".to_string(),
        "".to_string(),
        "".to_string(),
    ];
   
    let mut file = std::fs::File::create("nigerian_brewery_limited.txt").expect("create failed");

    file.write_all(b"Lager            | Stout          | Non-Alcoholic\n").expect("write failed");
    file.write_all(b"----------------------------------------------------\n");

    for i in 0..lager.len() {
        let c1 = lager.get(i).unwrap();
        let c2 = stout.get(i).unwrap();
        let c3 = non_alcoholic.get(i).unwrap();

        let line = format!("{:<16} | {:<14} | {}\n", c1, c2, c3);

        file.write_all(line.as_bytes()).expect("write failed");
    };

    println!("file created successfully")
}
