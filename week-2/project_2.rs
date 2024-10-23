fn main() {
    let toshiba:f64 = 450_000.00 * 2.0;
    let mac:f64 = 1_500_000.00;
    let hp:f64 = 750_000.00 * 3.0;
    let dell:f64 = 2_850_000.00 * 3.0;
    let acer:f64 = 250_00.00;

    let sum = toshiba + mac + hp + dell + acer;
    println!("Total sum = {}", sum);
    let average = sum / 10.0;
    println!("Average price = {}", average);
}