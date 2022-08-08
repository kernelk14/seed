#[allow(unused_imports)]
use std::collections::{self, VecDeque};

#[derive(Debug)]
struct Stack<isize> {
    stack: Vec<isize>,
}

impl Stack<isize> {
    fn new() -> Self {
        Stack { stack: Vec::new() }    
    }
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn push(&mut self, item: isize) {
        self.stack.push(item)
    }
    fn peek(&self) {
        self.stack.last();
    }
}

fn tokenize(program: String) {
    // let s: Stack<i32> = Stack { stack: Vec::new() };
    let mut stack: Stack<isize> = Stack::new();
    for op in program.split(" ") { 
        if op == "write" {
            println!("{}      : A Write Instruction", op);
            let a = Some(stack.pop()).unwrap();
            println!("{:?}", a);
        } else {
            stack.push(op.parse::<isize>().unwrap());
            println!("{:?}          : Stack Contents", stack.peek());
        }
        println!("{:?}         : Next", stack.peek());
    }
}

fn main() {
    let program: String = "6 write 7 write".to_string();
    tokenize(program);
}
