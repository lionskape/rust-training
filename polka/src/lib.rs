#![forbid(unsafe_code)]

use crate::Value::{Number, Symbol};
use std::{collections::HashMap, fmt::Display};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Symbol(sym) => write!(f, "'{}", sym),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Interpreter {
    s: Vec<Value>,
    v: HashMap<String, Value>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            s: vec![],
            v: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &str) {
        let prep = expr
            .split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        for s in prep {
            match s {
                "+" => {
                    if let Number(a) = self.s.pop().unwrap() {
                        if let Number(b) = self.s.pop().unwrap() {
                            self.s.push(Number(a + b));
                            continue;
                        }
                    }
                    panic!("bad '+' arguments");
                }
                "-" => {
                    if let Number(a) = self.s.pop().unwrap() {
                        if let Number(b) = self.s.pop().unwrap() {
                            self.s.push(Number(a - b));
                            continue;
                        }
                    }
                    panic!("bad '-' arguments");
                }
                "*" => {
                    if let Number(a) = self.s.pop().unwrap() {
                        if let Number(b) = self.s.pop().unwrap() {
                            self.s.push(Number(a * b));
                            continue;
                        }
                    }
                    panic!("bad '*' arguments");
                }
                "/" => {
                    if let Number(a) = self.s.pop().unwrap() {
                        if let Number(b) = self.s.pop().unwrap() {
                            if b == 0f64 {
                                panic!("division by zero");
                            }
                            self.s.push(Number(a / b));
                            continue;
                        }
                    }
                    panic!("bad '/' arguments");
                }
                "set" => {
                    if let Symbol(sym) = self.s.pop().unwrap() {
                        self.v.insert(sym, self.s.pop().unwrap());
                        continue;
                    }
                    panic!("symbol expected for 'set'");
                }
                _ if s.starts_with('\'') => {
                    self.s.push(Symbol(s[1..].to_owned()));
                }
                _ if s.starts_with('$') => {
                    let num = self
                        .v
                        .get(&s[1..])
                        .unwrap_or_else(|| panic!("no such variable as {}", s));
                    self.s.push(num.to_owned());
                }
                _ => {
                    let num = s.parse().unwrap_or_else(|_| panic!("unknown symbol {}", s));
                    self.s.push(Number(num));
                }
            }
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.s.as_slice()
    }
}
