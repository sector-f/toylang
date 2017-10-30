extern crate itertools;

extern crate liner;
use liner::{Buffer, Context, KeyBindings};

use std::collections::HashMap;
use std::env::args_os;
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::path::Path;
use std::process::exit;

mod parser;
use parser::*;

mod ast;
use ast::*;

type VarMap = HashMap<String, Value>;

fn main() {
    let args = args_os().collect::<Vec<_>>();
    let parameters = args.iter().map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>();
    let exit_val;

    match args.len() {
        0 | 1 => {
            exit_val = repl();
        },
        _ => {
            let filename = &args[1].clone();
            exit_val = run_script(filename, parameters);
        },
    }
    exit(exit_val);
}

fn run_script<P: AsRef<Path>>(path: P, arguments: Vec<String>) -> i32 {
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

    match ast(&buf) {
        Ok(statements) => {
            let mut global_vars = HashMap::new();
            let arg_values = arguments.into_iter().map(|s| Value::String(s)).collect();
            global_vars.insert("ARGV".to_owned(), Value::Array(arg_values));
            for s in statements {
                if let Err(e) = run_statement(&mut global_vars, s) {
                    eprintln!("Error: {}", e);
                    return 1;
                }
            }
            return 0;
        },
        Err(e) => {
            eprintln!("Syntax error: {}", e);
            return 1;
        }
    }
}

fn run_statement(mut global_vars: &mut VarMap, statement: Statement) -> Result<(), String> {
    match statement {
        Statement::DeclareVar(name, expr) => {
            let value = eval_expr(&global_vars, &expr)?;
            global_vars.insert(name, value);
        },
        Statement::MutateVar(op, name, expr) => {
            if let None = global_vars.get(&name) {
                return Err(format!("undeclared variable: {}", name));
            }

            let old_value = global_vars.get(&name).unwrap().clone();
            let rhs = eval_expr(&global_vars, &expr)?;

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
                        return Err(format!("= is the only valid assignment operator for {}", old_value.get_type()));
                    }
                },
            }
        },
        Statement::Expression(expression) => {
            eval_expr(&global_vars, &expression)?;
        }
        Statement::If(if_s, elif_s, else_s) => {
            let if_cond = eval_expr(&global_vars, &if_s.e)?;
            if let Value::Boolean(b) = if_cond {
                if b {
                    for s in if_s.s {
                        run_statement(&mut global_vars, s)?;
                    }
                    return Ok(());
                } else if let Some(statements) = elif_s {
                    'else_if: for statement in statements {
                        if let Value::Boolean(b) = eval_expr(&global_vars, &statement.e)? {
                            if b {
                                for s in statement.s {
                                    run_statement(&mut global_vars, s)?;
                                }
                                return Ok(());
                            }
                        } else {
                            return Err(format!("expected boolean, found {}", if_cond.get_type()));
                        }
                    }
                }
                if let Some(statements) = else_s {
                    for s in statements {
                        run_statement(&mut global_vars, s)?;
                    }
                    return Ok(());
                }
            } else {
                return Err(format!("expected boolean, found {}", if_cond.get_type()));
            }
        },
        Statement::While(condition, statements) => {
            loop {
                let condition = eval_expr(&global_vars, &condition)?;
                if let Value::Boolean(b) = condition {
                    if b {
                        for s in statements.clone() {
                            run_statement(&mut global_vars, s)?;
                        }
                    } else {
                        break;
                    }
                } else {
                    return Err(format!("expected boolean, found {}", condition.get_type()));
                }
            }
        },
        Statement::Print(exprs) => {
            let values = exprs.into_iter().map(|expr| eval_expr(&global_vars, &expr)).collect::<Result<Vec<Value>, _>>()?;
            for val in values {
                let _ = stdout().write_all(val.to_string().as_bytes());
            }
            let _ = stdout().flush();
        },
        Statement::Println(exprs) => {
            let values = exprs.into_iter().map(|expr| eval_expr(&global_vars, &expr)).collect::<Result<Vec<Value>, _>>()?;
            for val in values {
                let _ = stdout().write_all(format!("{}", val).as_bytes());
            }
            let _ = stdout().write_all("\n".as_bytes());
            let _ = stdout().flush();
        },
        Statement::Exit(e) => {
            let status = eval_expr(&global_vars, &e)?;
            if let Value::Num(exit_val) = status {
                exit(exit_val as i32);
            } else {
                eprintln!("tried to exit with {} (number required)", status.get_type());
                exit(0);
            }
        },
    }

    Ok(())
}

fn eval_expr(global_vars: &VarMap, expr: &Expr) -> Result<Value, String> {
    match *expr {
        Expr::Literal(ref v) => {
            Ok(v.to_owned())
        },
        Expr::Reference(ref r) => {
            global_vars.get(r).map(|item| item.clone()).ok_or(format!("Undefined variable: {}", r))
        },
        Expr::Array(ref exprs) => {
            let mut array = Vec::new();
            for e in exprs {
                array.push(eval_expr(global_vars, e)?);
            }
            Ok(Value::Array(array))
        }
        Expr::Index(ref expression, ref index) => {
            let var = eval_expr(global_vars, expression)?;
            let index = eval_expr(global_vars, index)?;

            if let Value::Num(ref i) = index {
                match var {
                    Value::Array(ref values) => {
                        if let Some(item) = values.get(i.clone() as usize) {
                            Ok(item.clone())
                        } else {
                            Err(format!("attempted to access index {} of array with length of {}", index, values.len()))
                        }
                    },
                    _ => {
                        Err(format!("attempted to index a {}", var.get_type()))
                    }
                }
            } else {
                Err(format!("{} cannot be used as index", index.get_type()))
            }
        },
        Expr::BinOp(ref op, ref left, ref right) => {
            let left = eval_expr(global_vars, left)?;
            let right = eval_expr(global_vars, right)?;

            if let (&Value::Num(n1), &Value::Num(n2)) = (&left, &right) {
                Ok(Value::Num(
                    match *op {
                        Op::Add => n1 + n2,
                        Op::Sub => n1 - n2,
                        Op::Mul => n1 * n2,
                        Op::Div => n1 / n2,
                        Op::Mod => n1 % n2,
                        Op::Exp => n1.powf(n2),
                    }
                ))
            } else if let (&Value::String(ref left_val), &Value::String(ref right_val)) = (&left, &right) {
                let new_val = match *op {
                    Op::Add => format!("{}{}", left_val, right_val),
                    _ => {
                        return Err(format!("invalid operation ({} with {})", left.get_type(), right.get_type()));
                    },
                };
                Ok(Value::String(new_val))
            } else {
                Err(format!("invalid operation ({} with {})", left.get_type(), right.get_type()))
            }
        },
        Expr::Comparison(ref op, ref left, ref right) => {
            let left = eval_expr(global_vars, left)?;
            let right = eval_expr(global_vars, right)?;

            let compare_bools = |left: f64, right: f64| {
                match *op {
                    CompOp::Equal => left == right,
                    CompOp::NotEq => left != right,
                    CompOp::Gt => left > right,
                    CompOp::Ge => left >= right,
                    CompOp::Lt => left < right,
                    CompOp::Le => left <= right,
                }
            };

            let compare_strs = |left: &str, right: &str| {
                match *op {
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
        Expr::BoolChain(ref op, ref left, ref right) => {
            let left = eval_expr(global_vars, left)?;
            let right = eval_expr(global_vars, right)?;

            if let (&Value::Boolean(b1), &Value::Boolean(b2)) = (&left, &right) {
                Ok(Value::Boolean(
                    match *op {
                        BoolLogic::And => b1 && b2,
                        BoolLogic::Or => b1 || b2,
                    }
                ))
            } else {
                Err(format!("invalid boolean logic (expected two booleans, found {} and {})", left.get_type(), right.get_type()))
            }
        }
        Expr::UnOp(ref op, ref expr) => {
            let expr = eval_expr(global_vars, expr)?;
            match *op {
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

    loop {
        match context.read_line("> ", &mut |_| {}) {
            Ok(line) => {
                match single_line(&line) {
                    Ok(parsed) => {
                        match  parsed {
                            Line::Statement(s) => {
                                if let Err(e) = run_statement(&mut var_map, s) {
                                    println!("{}", e);
                                }
                            },
                            Line::Expression(e) => {
                                match eval_expr(&var_map, &e) {
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
