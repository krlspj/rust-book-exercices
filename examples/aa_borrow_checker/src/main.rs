fn bar (foo: &String){
    println!("-> {foo}, {:p}", foo);
}

fn bar1 (foo: String){
    println!("bar1-> {foo}, {:p}", &foo);
}

fn main() {
    let foo = String::from("hello");
    println!("main -> foo: {:p}", &foo);
    bar(&foo);
    dbg!(foo);

    let foo = String::from("bye");
    let foo_ptr: &str = &foo;
    println!("main -> let foo: {:p}", &foo);
    println!("foo_ptr: {:p}", foo_ptr);
    println!("String content: {}", foo_ptr);
    bar1(foo.clone());
    dbg!(foo);

    let a: i32 = 54;
    let a_ptr: *const i32 = &a as *const i32;
    println!("a_ptr {:p}", a_ptr);


    let foo = String::from("hello world!");
    unsafe {
        let mut mfptr: *const u8 = foo.as_ptr();
        println!("mut ptr {:p}", mfptr);
        println!("mut ptr {:p}", mfptr.offset(1));
        mfptr = mfptr.offset(2);
        println!("mut ptr {:p}", mfptr);

    }

//  this won't work because ownership of the foo variable will be lost in bar1
//  so u need to pass by reference or clone / copy the variable
//    let foo = String::from("bye");
//    bar1(foo);
//    dbg!(foo);

}

