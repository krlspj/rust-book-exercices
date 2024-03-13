use std::u32;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess2: u32 = 45;

    println!("guess {guess} guess2 {guess2}");
}
