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
        // compiler.optimize();
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
///
///  Label to store the address
///
struct  Label(usize);
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
    /// Instruction Pointer
    ip: usize,
    /// Labels {stores start + 1 address for every ']'(End of the loop)}
    labels: Vec< Label>,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            memory: [0; MEMORY_SIZE].to_vec(),
            ip: 0,
            labels: Vec::new(),
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

    // fn optimize(&self) {}
    fn jump_to_starting_of_the_loop(&mut self) {
        let jmp_addr = self.labels.last();
        if jmp_addr.is_some() {
            self.ip = jmp_addr.unwrap().0;
        } // else ignore it. and think ']' as a comment.
    }

    fn evaluate(&mut self) {
        let mut memory_index: usize = 0;
        let tokens_len = self.tokens.len();

        while self.ip != tokens_len {
            if tokens_len == 0 {
                break;
            }
            let lexem = &self.tokens[self.ip];
            match lexem {
                Token::Inc(i) => {
                    if memory_index < MEMORY_SIZE {
                        memory_index += i
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
                    // ip + 1 so, it would loop jmp to '[' + 1
                    self.labels.push( Label(self.ip + 1));
                },
                Token::RSquare => {
                    if self.memory[memory_index] != 0 {
                        self.jump_to_starting_of_the_loop();
                        continue;
                    } else {
                        // pop out start  Label which was currently executing.
                        self.labels.pop();
                    }
                }
            }
            // print!("lexeme_index = {}", lexeme_index);
            self.ip += 1;
        }
    }
}
