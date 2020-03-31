use std::io;
use std::io::prelude::*;
mod interpreter;

const MEMORY_SIZE: usize = 1000;

fn main() {
    println!("It's a BF Interpreter, type quit and ENTER to exit.");
    loop {
        print!("||}}");
        io::stdout().flush().expect("\n");
        // taking input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("something went wrong.");
        if input.trim() == "quit" {
            // If the input is "quit", the program will quit.
            std::process::exit(0);
        }
        let mut compiler = Lexer::new();
        compiler.scan(input);
        compiler.optimize();
        compiler.evaluate();
    }
}

enum Token {
    /// > Increment data pointer
    Inc(usize),    
    /// < Decrement data pointer
    Dec(usize),    
    /// + data Plus one
    Plus(u8),
    /// - data Minus one
    Minus(u8),     
    /// . output the data
    PutChar,
    /// , take input of data of a byte.         
    GetChar,
    /// [ start loop from ']'
    LSquare,      
    /// ] end if data is zero
    RSquare,      
}

struct Lexer {
    /// This will store the tokens
    tokens: Vec<Token>,
    /// Memory to store the 
    memory: Vec<u8>,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            memory: [0; MEMORY_SIZE].to_vec(),
        }
    }

    fn scan(&mut self, code: String) {
        for symbol in code.chars() {
            match symbol {
                '>' => &self.tokens.push(Token::Inc(1)),
                '<' => &self.tokens.push(Token::Dec(1)),
                '+' => &self.tokens.push(Token::Plus(1)),
                '-' => &self.tokens.push(Token::Minus(1)),
                '.' => &self.tokens.push(Token::PutChar),
                ',' => &self.tokens.push(Token::GetChar),
                '[' => &self.tokens.push(Token::LSquare),
                ']' => self.tokens.push(Token::RSquare),
                 _ => {}
            }
        }
    }

    fn optimize(&self) {

    }

    fn evaluate(&mut self) {
        let mut memory_index: usize = 0;
        let mut lexeme_index: usize = 0;
        let lexeme_len = self.tokens.len();
        let mut loops_jmp_points = Vec::new();

        while lexeme_index != lexeme_len {
            if lexeme_len == 0 {
                break;
            }
            let lexem = &self.tokens[lexeme_index];
            match lexem {
                Token::Inc(i) => memory_index += i,
                Token::Dec(i) => memory_index -= i,
                Token::Plus(i) => {
                    if self.memory[memory_index] < 255 {
                        self.memory[memory_index] += i;
                    }
                },
                Token::Minus(i) => {
                    if self.memory[memory_index] > 0 {
                        self.memory[memory_index] -= i;
                    }
                },
                Token::PutChar => {
                    print!("\n{}", self.memory[memory_index] as char);
                },
                Token::GetChar => {
                    let mut chr = String::new();
                    io::stdin().read_line(&mut chr).expect("something");
                    print!("chr {}", chr);
                    self.memory[memory_index] = chr.chars().next().unwrap() as u8;
                },
                Token::LSquare => {
                    loops_jmp_points.push(lexeme_index + 1);
                },
                Token::RSquare => {
                    if self.memory[memory_index] != 0 {
                        let temp_index = loops_jmp_points.last();
                        if temp_index.is_none() {
                            println!("syntax error: loop never started but ended.");
                        }
                        lexeme_index = *temp_index.unwrap();
                        continue;
                    }
                }
            }
            // print!("lexeme_index = {}", lexeme_index);
            lexeme_index += 1;
        }
    }
}
