use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Integer(u32),
    Word(String),
}

impl Token {
    fn value(&self) -> u32 {
        match *self {
            Token::Integer(i) => i,
            Token::Word(_) => panic!("Not a number!"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Integer(ref i) => write!(f, "{}", i),
            Token::Word(ref word) => write!(f, "{}", word),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Print,
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Rotate,
    Over,
    Swap,
    Drop,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut forth_file = File::open(&args[1]).unwrap();

    let mut line = String::new();
    forth_file.read_to_string(&mut line).unwrap();

    let t_tokens: Vec<&str> = line.split_whitespace().collect();
    let mut tokens = Vec::new();

    let mut dict: HashMap<String, Op> = HashMap::new();

    dict.insert(".".to_string(), Op::Print);
    dict.insert("+".to_string(), Op::Add);
    dict.insert("-".to_string(), Op::Sub);
    dict.insert("*".to_string(), Op::Mul);
    dict.insert("/".to_string(), Op::Div);
    dict.insert("DUP".to_string(), Op::Dup);
    dict.insert("ROT".to_string(), Op::Rotate);
    dict.insert("OVER".to_string(), Op::Over);
    dict.insert("SWAP".to_string(), Op::Swap);
    dict.insert("DROP".to_string(), Op::Drop);


    for token in t_tokens.iter() {
        let x = u32::from_str(token);

        if let Ok(i) = x {
            tokens.push(Token::Integer(i));
        } else {
            tokens.push(Token::Word(token.to_string()));
        };
    }

    let mut stack = Vec::new();

    for token in tokens.iter() {
        match *token {
            Token::Integer(_) => stack.push(token.value()),
            Token::Word(ref word) => {
                let op = dict[&word.to_string().to_uppercase()].clone();
                match op {
                    Op::Print => { println!("{} ", stack.pop().unwrap()); },
                    Op::Add => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(x + y);
                    },
                    Op::Sub => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(y - x);
                    },
                    Op::Mul => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(x * y);
                    },
                    Op::Div => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(y / x);
                    },
                    Op::Dup => {
                        let x = stack.pop().unwrap();
                        stack.push(x);
                        stack.push(x);
                    },
                    Op::Rotate => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        let z = stack.pop().unwrap();
                        stack.push(y);
                        stack.push(z);
                        stack.push(x);
                    },
                    Op::Over => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(x);
                        stack.push(y);
                        stack.push(x);
                    },
                    Op::Swap => {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        stack.push(y);
                        stack.push(x);
                    },
                    Op::Drop => {
                        stack.pop().unwrap();
                    },
                    //_ => { print!("Op {:?} not yet handled", op); },
                }
            },
        }
    }
}
