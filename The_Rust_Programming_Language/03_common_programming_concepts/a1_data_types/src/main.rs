use std::{io, u32};

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess2: u32 = 45;

    println!("guess {guess} guess2 {guess2}");

    let t: bool = true;
    let f: bool; // can't use uninitialized values

    f = false;
    println!("t -> {t}, f -> {f}");

    let c = 'z';
    println!("c -> {c}");

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("y -> {y}, tup: {:?}", tup);
    println!("tup elemen {}, {}, {}", tup.0, tup.1, tup.2);

    // array type
    let a0 = [1, 2, 3, 4, 5];
    let a1: [i32; 5] = [0, 1, 2, 3, 4];

    let a2 = [3; 7];

    println!("a0: {:?}, a1: {:?}, a2: {:?}", a0, a1, a2);

    println!("Please enter an array index, arr len: 5");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a0[index];

    println!("the value of the lement at index {index} is: {element}");
}
