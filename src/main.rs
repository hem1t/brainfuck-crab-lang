use std::io::{self, BufRead, Write};
use libc;

const MEMORY_SIZE: usize = 1000;

fn main() {
    let mut stdout = io::stdout();
    println!("It's a BF Interpreter, type quit and ENTER to exit.");
    print!(">>");
    stdout.flush().unwrap();
    // taking input
    let input = io::stdin();
    for line in input.lock().lines() {
        let code = line.unwrap();
        if code.trim() == "quit" {
            // If the input is "quit", the program will quit.
            return;
        }
        let mut compiler = Lexer::new();
        compiler.tokenize(code.trim().to_string());
        compiler.optimize();
        compiler.evaluate();
        print!("\n>>");
        stdout.flush().unwrap();
    }
}

/// # Token
///
/// Contains all tokens related to BrainFuck language.
///
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

/// # Lexer
///
/// `Lexer` will take the code and tokenize it, and evaluate it.
///
/// 
///
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

///
/// Tokenizes the given code, and will store in self.tokens
///
    fn tokenize(&mut self, code: String) {
        for symbol in code.chars() {
            match symbol {
                '>' => self.tokens.push(Token::Inc(1)),
                '<' => self.tokens.push(Token::Dec(1)),
                '+' => self.tokens.push(Token::Plus(1)),
                '-' => self.tokens.push(Token::Minus(1)),
                '.' => self.tokens.push(Token::PutChar),
                ',' => self.tokens.push(Token::GetChar),
                '[' => self.tokens.push(Token::LSquare),
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
                Token::Inc(i) => {
                    if memory_index < 10 {
                        memory_index += i
                    } else {
                        return;
                    }
                },
                Token::Dec(i) => {
                    if memory_index > 0 {
                        memory_index -= i;
                    }
                },
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
                    print!("{}", self.memory[memory_index] as char);
                    let mut stdout = io::stdout();
                    stdout.flush().unwrap();
                },
                Token::GetChar => {
                    let chr = unsafe { libc::getchar() };
                    self.memory[memory_index] = (chr & 0xff) as u8;
                },
                Token::LSquare => {
                    loops_jmp_points.push(lexeme_index+1);
                },
                Token::RSquare => {
                    if self.memory[memory_index] != 0 {
                        let temp_index = loops_jmp_points.last();
                        if temp_index.is_none() {
                            println!("syntax error: loop never started but ended.");
                            break;
                        }
                        lexeme_index = *temp_index.unwrap();
                        continue;
                    } else {
                        loops_jmp_points.pop();
                    }
                }
            }
            // print!("lexeme_index = {}", lexeme_index);
            lexeme_index += 1;
        }
    }
}
