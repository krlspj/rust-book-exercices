use chrono::Local;
use colored::*;
use rand::Rng;

mod build_info;

fn main() {
    let version = build_info::VERSION;
    let date = build_info::DATE;
    println!("* Version: {}            *", version);
    println!("* Date: {}            *", date);
    // Generate a random number between 1 and 10
    let num = rand::rng().random_range(1..=10);
    println!("{}", format!("Random number: {}", num).green());

    // Get current date and time
    let now = Local::now();
    println!(
        "{}",
        format!("Now: {}", now.format("%Y-%m-%d %H:%M:%S")).blue()
    );
    println!("new code 1!");
    println!("new code 2!");
    println!("new code 3!");
    println!("new code 4!");
    println!("new code 5!");
    println!("new code 6!");
}
//fn main() {
//    println!("Hello, world!");
//}
