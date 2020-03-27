#![allow(non_camel_case_types)]
use std::io;
use std::io::prelude::*;

fn main() {
    println!("It's a BF compiler, type quit and ENTER to exit.");
    loop {
        print!(">>");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("something went wrong.");
        if input.trim() == "quit" {
            std::process::exit(0);
        }
        let mut compile = lexer::new();
        compile.scan(input);
        compile.optimize();
        compile.evaluate();
    }
}

enum token {
    inc(usize),    // > Increment data pointer
    dec(usize),    // < Decrement data pointer
    plus(u8),      // + data plus one
    minus(u8),     // - data minus one
    point,         // . output the data
    comma,         // , take input of data of a byte.
    L_square,      // [ start loop from ']'
    R_square,      // ] end if data is zero
}

struct lexer {
    tokens: Vec<token>,
    counter: Vec<u8>,
}

impl lexer {
    fn new() -> lexer {
        lexer {
            tokens: Vec::new(),
            counter: [0; 1000].to_vec(),
        }
    }

    fn scan(&mut self, code: String) {
        for symbol in code.chars() {
            match symbol {
                '>' => {
                   &self.tokens.push(token::inc(1));
                },
                '<' => {
                   &self.tokens.push(token::dec(1));
                },
                '+' => {
                   &self.tokens.push(token::plus(1));
                },
                '-' => {
                   &self.tokens.push(token::minus(1));
                },
                '.' => {
                   &self.tokens.push(token::point);
                },
                ',' => {
                   &self.tokens.push(token::comma);
                },
                '[' => {
                   &self.tokens.push(token::L_square);
                },
                ']' => {
                   &self.tokens.push(token::R_square);
                },
                 _ => {}
            }
        }
    }

    fn optimize(&self) {

    }

    fn evaluate(&mut self) {
        let mut counter_index: usize = 0;
        let mut lexeme_index: usize = 0;
        let lexeme_len = self.tokens.len();
        let mut loops_jmp_points = Vec::new();
        let mut safe_count = 0;

        loop {
            if lexeme_len == 0 {
                break;
            }
            let lexem = &self.tokens[lexeme_index];
            match lexem {
                token::inc(i) => {
                    counter_index += i;
                },
                token::dec(i) => {
                    counter_index -= i;
                },
                token::plus(i) => {
                    if self.counter[counter_index] < 255 {
                        self.counter[counter_index] += i;
                    }
                },
                token::minus(i) => {
                    if self.counter[counter_index] > 0 {
                        self.counter[counter_index] -= i;
                    }
                },
                token::point => {
                    print!("\n{}", self.counter[counter_index] as char);
                },
                token::comma => {
                    let mut chr = String::new();
                    io::stdin().read_line(&mut chr).expect("something");
                    print!("chr {}", chr);
                    self.counter[counter_index] = chr.chars().next().unwrap() as u8;
                },
                token::L_square => {
                    loops_jmp_points.push(lexeme_index);
                },
                token::R_square => {
                    if self.counter[counter_index] != 0 {
                        let temp_index = loops_jmp_points.last();
                        if temp_index.is_none() {
                            println!("syntax error: loop never started but ended.");
                        }
                        lexeme_index = *temp_index.unwrap();
                    }
                }
            }
            lexeme_index += 1;
            if lexeme_index == lexeme_len {
                break;
            }
        } 
        print!("counter: {:?}", self.counter.get(0..5));
    }
}
