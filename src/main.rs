use libc;
use std::io::{self, BufRead, Write};

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

/// Match direction
enum Direction {
    Left,
    Right,
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
    /// Instruction pointer
    ip: usize,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            memory: [0; MEMORY_SIZE].to_vec(),
            ip: 0,
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

    fn optimize(&self) {}

    fn match_loop(&mut self, dir: Direction) {
        let mut depth = 1;
        let dir: i64 = match dir {
            Direction::Left => 1,
            Direction::Right => -1,
        };
        loop {
            self.ip = (self.ip as i64 + dir) as usize;

            match &self.tokens[self.ip] {
                Token::LSquare => depth += dir,
                Token::RSquare => depth -= dir,
                _ => (),
            };

            if depth <= 0 {
                break;
            }
        }
    }

    fn evaluate(&mut self) {
        let mut memory_index: usize = 0;
        let code_len = self.tokens.len();

        while self.ip != code_len {
            if code_len == 0 {
                break;
            }
            let token = &self.tokens[self.ip];
            match token {
                Token::Inc(i) => {
                    if memory_index < MEMORY_SIZE {
                        memory_index += i
                    }
                }
                Token::Dec(i) => {
                    if memory_index > 0 {
                        memory_index -= i;
                    }
                }
                Token::Plus(i) => {
                    if self.memory[memory_index] < 255 {
                        self.memory[memory_index] += i;
                    }
                }
                Token::Minus(i) => {
                    if self.memory[memory_index] > 0 {
                        self.memory[memory_index] -= i;
                    }
                }
                Token::PutChar => {
                    print!("{}", self.memory[memory_index] as char);
                    let mut stdout = io::stdout();
                    stdout.flush().unwrap();
                }
                Token::GetChar => {
                    let chr = unsafe { libc::getchar() };
                    self.memory[memory_index] = (chr & 0xff) as u8;
                }
                Token::LSquare => {
                    if self.memory[memory_index] == 0 {
                        self.match_loop(Direction::Left);
                    }
                }
                Token::RSquare => {
                    self.match_loop(Direction::Right);
                    self.ip -= 1;
                }
            }
            // print!("self.ip = {}", self.ip);
            self.ip += 1;
        }
    }
}
