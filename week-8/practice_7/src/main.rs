fn main() {
   let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14 , 100);
    println!("Tuple contains = {:?}", datatype_tuple);

    let no_datatype_tuple: (&str, &str, u16) =("Rust", "fun", 100);
    println!("Tuple contains = {:?}", no_datatype_tuple);

    println!("Value at index 0 = {}", datatype_tuple.0);

    println!("Value at index 1 = {}", datatype_tuple.1);

    println!("Value at index 2 = {}", datatype_tuple.2);
}
