#![feature(variant_count)]

use std::io::stdin;

use itertools::{Itertools, iproduct};
use num::rational::Ratio;
use operator::Operator;

mod expr;
mod operator;

const NUMBERS: usize = 4;
const OPERATORS: usize = NUMBERS - 1;
const LENGTH: usize = NUMBERS + OPERATORS;

fn main() {
    let stdin = stdin();

    println!("Enter {NUMBERS} numbers separated by spaces:");
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let q = buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_array::<4>()
        .unwrap();

    let ans = solve(q);
    for expr in ans {
        println!("{}", expr::infix(&expr));
    }
}

#[must_use]
fn solve(q: [i32; NUMBERS]) -> Vec<[expr::Item; LENGTH]> {
    // 逆ポーランド記法で有効となる長さ7の式のテンプレート
    const TEMPLATE: [[expr::ItemKind; LENGTH]; 5] = [
        [
            // infix: (((a <op1> b) <op2> c) <op3> d)
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
        ],
        [
            // infix: ((a <op1> b) <op3> (c <op2> d))
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
        ],
        [
            // infix: ((a <op2> (b <op1> c)) <op3> d)
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
        ],
        [
            // infix: (a <op3> ((b <op1> c) <op2> d))
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
        ],
        [
            // infix: (a <op2> (b <op3> (c <op1> d)))
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
        ],
    ];

    // テンプレート、数値の順列、演算子の組み合わせの直積を取る
    iproduct!(
        TEMPLATE.iter(),
        q.iter()
            .map(|&n| expr::Item::Number(Ratio::new(n, 1)))
            .permutations(NUMBERS),
        iproduct!(
            Operator::VALUES.into_iter().map(expr::Item::Operator),
            Operator::VALUES.into_iter().map(expr::Item::Operator),
            Operator::VALUES.into_iter().map(expr::Item::Operator),
        )
        .map(|(a, b, c)| [a, b, c])
    )
    // 式を生成
    .map(|(template, num, op)| {
        template
            .iter()
            .scan((0, 0), |(i, j), k| match k {
                expr::ItemKind::Number => {
                    *i += 1;
                    Some(num[*i - 1].clone())
                }
                expr::ItemKind::Operator => {
                    *j += 1;
                    Some(op[*j - 1].clone())
                }
            })
            .collect_array::<LENGTH>()
            .unwrap()
    })
    // 全探索して10になる式を抽出
    .filter(|expr| {
        expr::eval(expr)
            .map(|v| v == Ratio::new(10, 1))
            .unwrap_or(false)
    })
    .unique()
    .collect()
}
