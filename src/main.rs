#[allow(unused_imports)]
use std::collections::{self, VecDeque};

#[derive(Debug)]
struct Stack<i32> {
    stack: Vec<i32>,
}

impl Stack<i32> {
    fn new() -> Self {
        Stack { stack: Vec::new() }    
    }
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn push(&mut self, item: i32) {
        self.stack.push(item)
    }
    fn peek(&self) -> Option<&i32> {
        self.stack.last()
    }
}

fn tokenize(program: String) {
    // let s: Stack<i32> = Stack { stack: Vec::new() };
    let mut stack: Stack<i32> = Stack::new();
    for op in program.split(" ") {
        match op {
            "+" => {
                println!("{}         : A Plus Instruction", op);
                let a = Some(stack.pop());
                let b = Some(stack.pop());
                let f = |a, b|{
                    Some(a) + Some(b)
                };
                stack.push(a.and_then(|a| b.map(|b| f(a, b))));
            }
            "write" => println!("{}     : A Write Instruction", op),
            _ => {
                stack.push(op.parse::<i32>().unwrap());
                println!("{:?}   : Stack Contents", stack.peek());
            }
        }
    }
}

fn main() {
    let program: String = "5 6 + write".to_string();
    tokenize(program);
}
