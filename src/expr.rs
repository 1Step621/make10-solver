use std::fmt::Display;

use anyhow::Result;
use num::rational::Ratio;

use crate::Operator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ItemKind {
    Number,
    Operator,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Item {
    Number(Ratio<i32>),
    Operator(Operator),
}

impl Item {
    pub fn assert_number(&self) -> Ratio<i32> {
        match self {
            Item::Number(v) => *v,
            _ => panic!("Expected number"),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Number(n) => write!(f, "{}", n),
            Item::Operator(o) => write!(f, "{}", o),
        }
    }
}

// 式を評価する
pub fn eval(stack: &[Item]) -> Result<Ratio<i32>> {
    let mut mem = Vec::new();
    for item in stack {
        match item {
            Item::Number(n) => mem.push(Item::Number(*n)),
            Item::Operator(o) => {
                let b = mem.pop().unwrap().assert_number();
                let a = mem.pop().unwrap().assert_number();
                if b == Ratio::new(0, 1) && *o == Operator::Div {
                    return Err(anyhow::anyhow!("Division by zero"));
                }
                mem.push(Item::Number(o.apply(a, b)));
            }
        }
    }
    Ok(mem.pop().unwrap().assert_number())
}

// 式を逆ポーランドから通常の形式に変換する
pub fn infix(stack: &[Item]) -> String {
    let mut mem = Vec::new();
    for item in stack {
        match item {
            Item::Number(n) => mem.push(n.to_string()),
            Item::Operator(o) => {
                let b = mem.pop().unwrap();
                let a = mem.pop().unwrap();
                mem.push(format!("({} {} {})", a, o, b));
            }
        }
    }
    mem.pop().unwrap()
}
