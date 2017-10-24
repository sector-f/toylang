extern crate liner;
use liner::{Buffer, Context, KeyBindings};

mod parser;
use parser::*;

mod ast;
use ast::*;

// use std::collections::HashMap;

fn main() {
    let mut context = Context::new();
    context.completer = None;
    context.key_bindings = KeyBindings::Emacs;

    println!("Type `quit` to quit");

    loop {
        match context.read_line("> ", &mut |_| {}) {
            Ok(line) => {
                match &*line.trim() {
                    "quit" => {
                        break;
                    },
                    _ => {
                        match s_or_e(&line) {
                            Ok(n) => println!("{:?}", n),
                            Err(e) => println!("{}", e),
                        }
                    }
                }

                let buffer = Buffer::from(line);
                let _ = context.history.push(buffer);
            },
            Err(_) => {
                break;
            },
        }
    }
}
