fn bar (foo: &String){
    println!("-> {foo}");
}

fn bar1 (foo: String){
    println!("-> {foo}");
}

fn main() {
    let foo = String::from("hello");
    bar(&foo);
    dbg!(foo);

    let foo = String::from("bye");
    bar1(foo.clone());
    dbg!(foo);

//  this won't work because ownership of the foo variable will be lost in bar1
//  so u need to pass by reference or clone / copy the variable
//    let foo = String::from("bye");
//    bar1(foo);
//    dbg!(foo);

}

