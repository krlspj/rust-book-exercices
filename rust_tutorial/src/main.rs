#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::f32::consts::E;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    //num_1();
    //num_2();
    //integers();
    //booleans();
    precision();
}

fn precision() {
    let mut num_1: f32 = 1.1111111111111;
    num_1 += 1.0;
    println!("f32: {}", num_1 + 0.00000000000001);

    let num_2: f64 = 1.1111111111111;
    println!("f64: {}", num_2 + 0.0000000000001);
}

fn booleans() {
    let is_true = true; // false
    let my_grade = 'A';
}

fn integers() {
    // Unsigned integer : u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize

    println!("Max u32 \t: {}", u32::MAX);
    println!("Max u64 \t: {}", u64::MAX);
    println!("Max usize \t: {}", usize::MAX);
    println!("Max u128 \t: {}", u128::MAX);
    println!("Max f32 \t: {}", f32::MAX);
    println!("Max f64 \t: {}", f64::MAX);
}
fn num_2() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want {} $", age, ONE_MIL);
}

fn num_1() {
    println!("What is your name?");

    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    io::stdin()
        .read_line(&mut name)
        .expect("Didn't recieve input");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
