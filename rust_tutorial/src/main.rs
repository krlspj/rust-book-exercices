#![allow(unused)]
//#![allow(non_snake_case)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::E;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::{io, thread};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    //num_1();
    //num_2();
    //integers();
    //booleans();
    //precision();
    //random_num();
    //conditions();
    //arrays();
    //tuples();
    //variable_types();
    //unsafe { pointers_operations() };
    //strings();
    //enums();
    //vectors();
    //functions_0();
    //generics();
    //mem_0();
    //mem_strings();
    //hashmaps();
    //structs();
    //packages();
    //errors();
    //iterators_2();
    //closures();
    //smartPointers();
    //smart_pointers();
    //concurrency();
    concurrency_1();
}

fn concurrency_1() {
    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance: {} Withdrawl a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdraw {} Current balance: {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}

fn concurrency() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}

fn smart_pointers() {
    println!("--- Smart pointers ---");
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
    //   50
    //   /\
    // 35  40
}
fn closures() {
    // let var_name = |parameters| -> return_type {Body}
    let can_vote = |age: i32| age >= 18;
    println!("Can vote : {}", can_vote(8));

    let mut samp1 = 5;
    let print_var = || println!("- samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("-- samp1 = {}", samp1);
    samp1 = 10;
    println!("--- samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}

fn iterators_2() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}

fn errors() {
    //panic!("This a terrible error");
    //let lil_arr = [1, 2];
    //println!("--- {}", lil_arr[10]);

    // Result has 2 varients, Ok and Err
    // enum Result <T, E>{
    // Ok(T),
    // Err(E),
    // Where T represents the data typeof the value returns and E
    // the type of error

    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };

    write!(output, "Just some\nRandom words...").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("--- {}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}

fn packages() {
    order_food();
    // Crates: Modules that produce a library or executable
    // Modules: Organize and handle privacy
    // Packages: Build test and share crates
    // Paths: A way of naming an item sush a struct, function...
}

fn structs() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle0<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle0 {
        length: 4,
        height: 10.5,
    };

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle {
                length: length,
                width: width,
            };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    const PI: f32 = 3.141592;
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle {
                length: length,
                width: width,
            };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rect: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rectangle area: {}", rect.area());
    println!("Circle area: {}", circ.area());

    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: "Bob Smith".to_string(),

        address: String::from("555 Main St"),
        balance: 234.50,
    };
}

fn hashmaps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clar Kent");
    heroes.insert("Batman", "Burce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn mem_strings() {
    let mut str1: String = String::from("Krls");
    let str2: String = str1.clone();
    //print_str(str1);
    change_string(&mut str1);
    let str3 = print_return_str(str1);

    println!("str3 = {}", str3);
}

fn mem_0() {
    let str0: String = String::from("World");
    println!("--- str0 Hello {:p}", &str0);
    let str0_1: &String = &str0;
    let str1: String = "world".to_string();
    println!("--- str1 Hello {:p}", &str1);
    let str1_1 = str1;

    //println!("Hello {}", str0); // Borrow of moved value
    println!("--- str0_1 Hello {:p}", &str0_1);
    //println!("Hello {}", str1);
    println!("--- str1_1 Hello {:p}", &str1_1);
}

// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you request a certain amount of space.
// The OS finds space available and returns an address for that space called a pointer.

// Rules:
// 1. Each value has a variable that's called its owner
// 2. There is only one owner at a time
// 3. When the owner goes out of scope the value disappears

fn generics() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.6 + 4.4 = {}", get_sum_gen(5.6, 4.4));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn functions_0() {
    let resutl = get_sum(10, 20);
    println!("--- result: {}", resutl);

    let (a, b): (i32, i32) = get_2(4);
    println!("--- get_2: {}, {}", a, b);

    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list_0(&num_list))
}

fn sum_list_0(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in list {
        println!("i -> {:p}", i);
        sum += i;
    }
    sum
}
fn sum_list_1(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in list.iter().map(|x| x as *const i32) {
        println!("i -> {:p}", i);
        sum += unsafe { *i };
    }
    sum
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn get_sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st element : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("-> {}", i)
    }
    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
    println!("--- vect2: {:?}", vec2);
}

fn enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    println!("is today weekend? {}", today.is_weekend());
}

fn strings() {
    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str("strin word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");

    println!("{}", st2);

    let st3: String = String::from("x r t b h k k a m c");

    let mut v1: Vec<char> = st2.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let str4: &str = "Random string";

    let mut st5: String = str4.to_string();
    println!("{}", st5);

    let byte_arr1: &[u8] = st5.as_bytes();
    let st6: &str = &st5[0..6];
    println!("String length : {}", st6.len());
    st5.clear();
}

fn variable_types() {
    let mut i: i8 = 100;
    for idx in 100..200 {
        println!("i => {}", i);
        //i += 1;
        i = i.wrapping_add(1);
    }

    let mut i: i8 = 100;
    let mut ptr: *mut i8 = &mut i;

    for _ in 100..200 {
        println!("i ==> {}", unsafe { *ptr });

        //ptr = unsafe { ptr.offset(1) };

        unsafe {
            *ptr = *ptr.add(1);
        };
    }

    let mut i: i8 = 127;
    let ptr: *mut i8 = &mut i;

    unsafe {
        *ptr = *ptr.offset(1 - std::mem::size_of::<i8>() as isize);
    }
    println!("i unsafe=> i:{}, ptr:{:p}, *ptr:{}", i, ptr, unsafe {
        *ptr
    });

    //let mut i: i8 = 127;
    //i += 1;
    //println!("i -> {}", i);
    let mut i: i8 = 125;
    let ptr: *mut i8 = &mut i;
    println!("i unsafe 0 => i: {}, ptr: {:p}, *ptr: {}", i, ptr, unsafe {
        *ptr
    });
    unsafe {
        //*ptr = *ptr.offset(1);
        *ptr += 1;
        *ptr += 1;
    }
    i = i.wrapping_add(1);

    println!("i unsafe 1 => i: {}, ptr: {:p}, *ptr: {}", i, ptr, unsafe {
        *ptr
    });
}

unsafe fn pointers_operations() {
    let mut i: i8 = 125;
    let p: *mut i8 = &mut i;
    println!("i val: {}, i ref {}, p ref: {:p}, p val:{}", i, &i, p, *p);

    *p += 1;

    println!("i val: {}, p ref: {:p}, p val:{}", i, p, *p);
    p.add(1);
    //*p += 1;
    println!("i val: {}, p ref: {:p}, p val:{}", i, p, *p);

    let mut i: i8 = 120;
    let ptr: *mut i8 = &mut i;

    i = i.wrapping_add(1);
    i = i.wrapping_mul(4);

    println!("Result: {}", i);

    let mut i: i8 = 127;
    i += 1;
    println!("Result: {}", i);
}

fn tuples() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn arrays() {
    let arr_1: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("1st: {}", arr_1[0]);

    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: usize = 0;

    println!("start loop{{}}");
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while loop_idx < arr_2.len() {
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val {}", val);
    }
}

fn conditions() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday!!");
    } else if (age == 21) && (age == 50) {
        println!("........");
    } else {
        println!("no important...");
    }

    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("bla bla..."),
        65..=i32::MAX => println!("old"),
        _ => println!("Not an important bday"),
    }

    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("First time"),
    }
}

fn random_num() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
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
