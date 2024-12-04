fn main() {
    let mut colors = ["red", "green", "yellow", "White"];

    let sliced_colors = &mut colors[1..3];

    println!("First slice is {:?}", sliced_colors);

    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
