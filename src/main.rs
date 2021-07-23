
mod interpreter;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let config = interpreter::Config {
        script: parse_args(&args),
    };

    interpreter::interpret(config);
}

// Attempts to parse the arguments from the command line into a string script.
fn parse_args(args: &Vec<String>) -> String {

    if args.len() == 0 { 
        eprintln!("Error: Not enough arguments!");
        std::process::exit(0);
    }

    match &mut File::open(&args[0]) {
        Ok(file) => {

            let mut script: String = String::from("");
            file.read_to_string(&mut script).unwrap();
            script

        }
        Err(e) => {
            eprintln!("Error: Cannot open file! {}", e);
            std::process::exit(0);
        }
    }

}