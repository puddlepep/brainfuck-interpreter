use std::io::{Write, stdout};

// Configuration for the interpreter.
// script: The string data of the script.
pub struct Config {
    pub script: String,
}

pub struct Token {
    pub id: char,
    pub value: Option<usize>,
}

// Start simple: 8-bit cells, 16 of them.
// TODO (maybe) -- Configurable cell size and count for each script.

// Lexical analysis!!!! Tokenizing!!
pub fn lex(config: Config) -> Vec<Token> {

    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut open_loops: Vec<usize> = Vec::new();

    while i < config.script.len() {

        let cmd = config.script.chars().nth(i).unwrap();
        match cmd {

            '>' => {
                tokens.push(Token { id: '>', value: None });
            }

            '<' => {
                tokens.push(Token { id: '<', value: None });
            }

            '+' => {
                tokens.push(Token { id: '+', value: None });
            }

            '-' => {
                tokens.push(Token { id: '-', value: None });
            }

            '.' => {
                tokens.push(Token { id: '.', value: None });
            }

            ',' => {
                tokens.push(Token { id: ',', value: None });
            }

            '[' => {
                tokens.push(Token { id: '[', value: None });
                open_loops.push(tokens.len()-1);
            }

            ']' => {
                match open_loops.pop() {
                    Some(idx) => {
                        tokens.push(Token { id: ']', value: Some(idx) });
                        tokens[idx].value = Some(tokens.len()-1);
                    }
                    None => {
                        eprintln!("Syntax Error: Extra closed loop at line {}!", i);
                        std::process::exit(0);
                    }
                }
            }

            _ => ()

        }

        i += 1;

    }

    if open_loops.len() != 0 {
        eprintln!("Syntax Error: Unbalanced loop at line {}!", open_loops[0]);
        std::process::exit(0);
    }

    tokens
}

// Interprets a single script program.
pub fn interpret(config: Config) {

    let tokens = lex(config);

    let mut cells: [u8; 16] = [0; 16];
    let mut pointer: usize = 0;
    
    let mut i: usize = 0;
    while i < tokens.len() {

        let token = &tokens[i];
        match token.id {

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
            
            ',' => {                
                let mut c: String = String::from("");
                print!("\nInput> ");
                stdout().flush().unwrap();

                std::io::stdin().read_line(&mut c).unwrap();
                cells[pointer] = c.chars().nth(0).unwrap() as u8;
            }

            '[' => {
                if cells[pointer] == 0 {
                    i = token.value.unwrap();
                }
            }

            ']' => {
                if cells[pointer] != 0 {
                    i = token.value.unwrap();
                }
            }

            _ => ()
        }

        i += 1;

    }
    
    // Display cell and pointer info.
    println!("\n\nPointer: {}", pointer);

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