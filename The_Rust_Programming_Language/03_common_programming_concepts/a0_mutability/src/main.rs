
fn main(){
// x is constant by default
//    let x:i32 = 5;
//    println!("x: {x}");
//    x = 6;
//    println!("x: {x}");

    let mut y:i8 = 5;
    println!("y: {y}");
    y = 127; 
    println!("y: {y}");

    // constants
    #[warn(dead_code)]
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("3h -> {THREE_HOURS_IN_SECONDS}s");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    y = y.wrapping_add(1);
    println!("the y now is: {y}");
             
}            
             
