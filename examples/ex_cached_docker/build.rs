use std::fs::File;
use std::io::Write;

fn main() {
    // Fetch version, commit, and date from environment variables
    let version = option_env!("VERSION").unwrap_or("unknown").to_string();
    //let commit = option_env!("COMMIT").unwrap_or("unknown").to_string();
    let date = option_env!("DATE").unwrap_or("unknown").to_string();

    // Generate a file containing these values
    let mut file = File::create("src/build_info.rs").unwrap();
    writeln!(file, "pub const VERSION: &str = \"{}\";", version).unwrap();
    //writeln!(file, "pub const COMMIT: &str = \"{}\";", commit).unwrap();
    writeln!(file, "pub const DATE: &str = \"{}\";", date).unwrap();

    // Instruct Cargo to rebuild if environment variables change
    println!("cargo:rerun-if-env-changed=VERSION");
    //println!("cargo:rerun-if-env-changed=COMMIT");
    println!("cargo:rerun-if-env-changed=DATE");
}
