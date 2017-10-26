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

    let mut global_vars = HashMap::new();
    match ast(&buf) {
        Ok(statements) => {
            match run_program(&mut global_vars, statements) {
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

fn run_program(mut global_vars: &mut VarMap, tree: Vec<Statement>) -> Result<(), String> {
    for statement in tree {
        match statement {
            Statement::DeclareVar(name, expr) => {
                let value = parse_expr(&global_vars, expr)?;
                global_vars.insert(name, value);
            },
            Statement::ShadowVar(op, name, expr) => {
                if let None = global_vars.get(&name) {
                    return Err(format!("undeclared variable: {}", name));
                }

                let old_value = global_vars.get(&name).unwrap().clone();
                let rhs = parse_expr(&global_vars, expr)?;

                match op {
                    AssignOp::Equals => {
                        global_vars.insert(name, rhs);
                    },
                    _ => {
                        if let (&Value::Num(old), &Value::Num(new)) = (&old_value, &rhs) {
                            let new_value;
                            match op {
                                AssignOp::AddEq => {
                                    new_value = old + new;
                                },
                                AssignOp::SubEq => {
                                    new_value = old - new;
                                },
                                AssignOp::MulEq => {
                                    new_value = old * new;
                                },
                                AssignOp::DivEq => {
                                    new_value = old / new;
                                },
                                AssignOp::ModEq => {
                                    new_value = old % new;
                                },
                                _ => unreachable!(),
                            }
                            global_vars.insert(name, Value::Num(new_value));
                        } else {
                            return Err(format!("= is only valid assignment for {}", old_value.get_type()));
                        }
                    },
                }
            },
            Statement::If(condition, statements) => {
                let condition = parse_expr(&global_vars, condition)?;
                if let Value::Boolean(b) = condition {
                    if b {
                        if let Err(e) = run_program(&mut global_vars, statements) {
                            return Err(e);
                        }
                    }
                } else {
                    return Err(format!("expected boolean, found {}", condition.get_type()));
                }
            },
            Statement::Print(e) => {
                let variable = parse_expr(&global_vars, e)?;
                println!("{}", variable);
            },
        }
    }

    Ok(())
}

fn parse_expr(global_vars: &VarMap, expr: Expr) -> Result<Value, String> {
    match expr {
        Expr::Literal(v) => {
            Ok(v)
        },
        Expr::Reference(r) => {
            global_vars.get(&r).map(|item| item.clone()).ok_or(format!("Undefined variable: {}", &r))
        },
        Expr::BinOp(op, left, right) => {
            let left = parse_expr(&global_vars, *left)?;
            let right = parse_expr(&global_vars, *right)?;

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
                Err(format!("invalid math ({} with {})", left.get_type(), right.get_type()))
            }
        },
        Expr::Comparison(op, left, right) => {
            let left = parse_expr(&global_vars, *left)?;
            let right = parse_expr(&global_vars, *right)?;

            let compare_bools = |left: f64, right: f64| {
                match op {
                    CompOp::Equal => left == right,
                    CompOp::NotEq => left != right,
                    CompOp::Gt => left > right,
                    CompOp::Ge => left >= right,
                    CompOp::Lt => left < right,
                    CompOp::Le => left <= right,
                }
            };

            let compare_strs = |left: &str, right: &str| {
                match op {
                    CompOp::Equal => left == right,
                    CompOp::NotEq => left != right,
                    CompOp::Gt => left > right,
                    CompOp::Ge => left >= right,
                    CompOp::Lt => left < right,
                    CompOp::Le => left <= right,
                }
            };

            if let (&Value::Num(n1), &Value::Num(n2)) = (&left, &right) {
                Ok(Value::Boolean(compare_bools(n1, n2)))
            } else if let (&Value::String(ref s1), &Value::String(ref s2)) = (&left, &right) {
                Ok(Value::Boolean(compare_strs(s1, s2)))
            } else {
                Err(format!("invalid comparison ({} with {})", left.get_type(), right.get_type()))
            }
        },
        Expr::BoolChain(op, left, right) => {
            let left = parse_expr(&global_vars, *left)?;
            let right = parse_expr(&global_vars, *right)?;

            if let (&Value::Boolean(b1), &Value::Boolean(b2)) = (&left, &right) {
                Ok(Value::Boolean(
                    match op {
                        BoolLogic::And => b1 && b2,
                        BoolLogic::Or => b1 || b2,
                    }
                ))
            } else {
                Err(format!("invalid boolean logic (expected two booleans, found {} and {})", left.get_type(), right.get_type()))
            }
        }
        Expr::UnOp(op, expr) => {
            let expr = parse_expr(&global_vars, *expr)?;
            match op {
                UnaryOp::Not => {
                    if let &Value::Boolean(b) = &expr {
                        Ok(Value::Boolean(!b))
                    } else {
                        Err(format!("cannot negate {}", expr.get_type()))
                    }
                },
            }
        }
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
                                                println!("Error: {}", e);
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
