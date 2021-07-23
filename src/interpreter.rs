use std::io::{Write, stdout};

// Configuration for the interpreter.
// script: The string data of the script.
pub struct Config {
    pub script: String,
}

// Start simple: 8-bit cells, 16 of them.
// TODO (maybe) -- Configurable cell size and count for each script.

// Interprets a single script program.
pub fn interpret(config: Config) {

    println!();
    let mut cells: [u8; 16] = [0; 16];
    let mut pointer: usize = 0;
    let mut scope: usize = 0;
    
    let mut i: usize = 0;
    while i < config.script.len() {

        let character = config.script.chars().nth(i).unwrap();
        match character {

            '>' => {
                pointer += 1;
                if pointer >= cells.len() { pointer = 0; }
            }

            '<' => {
                if pointer == 0 { pointer = cells.len() }
                pointer -= 1;
            }

            '+' => {
                cells[pointer] = cells[pointer].wrapping_add(1);
            }

            '-' => {
                cells[pointer] = cells[pointer].wrapping_sub(1);
            }

            '.' => {
                print!("{}", cells[pointer] as char);
                stdout().flush().unwrap();
            }

            '[' => {
                if cells[pointer] == 0 {
                    
                    let target_scope = scope;
                    let mut success = false;
                    while i < config.script.len() - 1 {

                        i += 1;

                        let c = config.script.chars().nth(i).unwrap();
                        match c {
                            '[' => {
                                scope += 1;
                            }

                            ']' => {
                                if scope == target_scope {
                                    success = true;
                                    break;
                                }
                                scope -= 1;
                            }

                            _ => ()
                        }
                    }

                    if !success {
                        println!("Syntax Error: Mismatched open bracket!");
                        std::process::exit(0);
                    }

                }
                else {
                    scope += 1;
                }
            }

            ']' => {
                if cells[pointer] != 0 {

                    let target_scope = scope;
                    let mut success = false;
                    while i > 0 {
                        
                        i -= 1;

                        let c = config.script.chars().nth(i).unwrap();
                        match c {
                            '[' => {
                                if scope == target_scope {
                                    success = true;
                                    break;
                                }
                                scope += 1;
                            }

                            ']' => {
                                scope -= 1;
                            }

                            _ => ()
                        }

                    }

                    if !success {
                        println!("Syntax Error: Mismatched closed bracket!");
                        std::process::exit(0);
                    }
                }
                else {
                    scope -= 1;
                }
            }

            _ => ()
        }

        i += 1;

    }
    
    // Display cell and pointer info.
    println!("\nPointer: {}", pointer);

    for cell in cells {
        print!("[{:03}] ", cell);
    }
    println!();

    print!("  ");
    for _ in 0..pointer {
        print!("      ");
    }
    print!("^\n");

}