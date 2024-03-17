use std::u32;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess2: u32 = 45;

    println!("guess {guess} guess2 {guess2}");

    let t: bool = true;
    let f: bool; // can't use uninitialized values

    f = false;
    println!("t -> {t}, f -> {f}");
}
