use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    let num_args = args.len();

    println!("Number of command-line arguments: {}", num_args);
    println!("Arguments: {:?}", &args);
    println!("Arguments: {:?}", &args[1..]);

    for arg in args.iter().skip(1) {
        println!("Argument: {}", arg);
    }
}
