extern crate itertools;

extern crate liner;
use liner::{Buffer, Context, KeyBindings};

use std::collections::HashMap;
use std::env::args_os;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

mod parser;
use parser::*;

mod ast;
use ast::*;

type VarMap = HashMap<String, Value>;

fn main() {
    let args = args_os().skip(1).collect::<Vec<_>>();
    let exit_val;

    match args.len() {
        0 => {
            exit_val = repl();
        },
        1 => {
            exit_val = run_script(&args[0]);
        }
        _ => {
            eprintln!("Usage: toylang [FILE]");
            exit_val = 1;
        },
    }
    exit(exit_val);
}

fn run_script<P: AsRef<Path>>(path: P) -> i32 {
    let path = path.as_ref();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        },
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        },
    }

    let mut var_map: VarMap = HashMap::new();
    match ast(&buf) {
        Ok(statements) => {
            match run_program(&mut var_map, statements) {
                Ok(_) => {
                    return 0;
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return 1;
                },
            }
        },
        Err(e) => {
            eprintln!("Syntax error: {}", e);
            return 1;
        }
    }
}

fn run_program(var_map: &mut VarMap, tree: Vec<Statement>) -> Result<(), String> {
    for statement in tree {
        match statement {
            Statement::AssignVar(s, e) => {
                let variable = parse_expr(&var_map, e)?;
                var_map.insert(s, variable);
            },
            Statement::ShadowVar(s, e) => {
                let variable = parse_expr(&var_map, e)?;
                var_map.insert(s, variable);
            },
            Statement::Print(e) => {
                let variable = parse_expr(&var_map, e)?;
                println!("{}", variable);
            },
        }
    }

    Ok(())
}

fn parse_expr(vars: &VarMap, expr: Expr) -> Result<Value, String> {
    match expr {
        Expr::Literal(v) => {
            Ok(v)
        },
        Expr::Reference(r) => {
            vars.get(&r).map(|item| item.clone()).ok_or(format!("Undefined variable: {}", &r))
        },
        Expr::Op(op, left, right) => {
            let left = parse_expr(&vars, *left)?;
            let right = parse_expr(&vars, *right)?;

            let math = |left: f64, right: f64| {
                match op {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mul => left * right,
                    Op::Div => left / right,
                    Op::Mod => left % right,
                    Op::Exp => left.powf(right),
                }
            };

            if let (&Value::Num(n1), &Value::Num(n2)) = (&left, &right) {
                Ok(Value::Num(math(n1, n2)))
            } else {
                Err(format!("Attempted to perform math with {} and {}", left.get_type(), right.get_type()))
            }
        },
    }
}

fn repl() -> i32 {
    let mut var_map: VarMap = HashMap::new();

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
                        match single_line(&line) {
                            Ok(parsed) => {
                                match  parsed {
                                    Line::Statement(s) => {
                                        if let Err(e) = run_program(&mut var_map, vec![s]) {
                                            println!("{}", e);
                                        }
                                    },
                                    Line::Expression(e) => {
                                        match parse_expr(&var_map, e) {
                                            Ok(expr) => {
                                                println!("{}", expr);
                                            },
                                            Err(e) => {
                                                println!("{}", e);
                                            },
                                        }
                                    },
                                }
                            },
                            Err(e) => {
                                println!("{}", e);
                            },
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

    0
}
