fn main() {
    let mut mountain_heights = ("Everset", 8848, "Fishtail", 6993);

    println!("Orgiginal tuple = {:?}", mountain_heights);

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}", mountain_heights);
}
