fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Antlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Nmae: {}, \nAddress: {}",uni,addr);

    let department:&'static str ="Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school);
}
