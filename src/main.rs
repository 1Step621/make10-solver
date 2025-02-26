#![feature(variant_count)]

use std::io::stdin;

use itertools::{Itertools, iproduct};
use num::rational::Ratio;
use operator::Operator;

mod operator;
mod expr;

fn main() {
    let stdin = stdin();

    println!("Enter 4 numbers separated by spaces:");
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let q = buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_array::<4>()
        .unwrap();

    solve(q);
}

fn solve(q: [i32; 4]) {
    // 逆ポーランド記法で有効となる長さ7の式のテンプレート
    const TEMPLATE: [[expr::ItemKind; 7]; 4] = [
        [
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
        ],
        [
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
        ],
        [
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Number,
            expr::ItemKind::Operator,
            expr::ItemKind::Operator,
        ],
        [
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
            .permutations(4),
        iproduct!(
            Operator::VALUES.iter(),
            Operator::VALUES.iter(),
            Operator::VALUES.iter()
        )
        .map(|(o1, o2, o3)| vec![
            expr::Item::Operator(*o1),
            expr::Item::Operator(*o2),
            expr::Item::Operator(*o3),
        ]),
    )
    // 式を生成
    .map(|(template, num, op)| {
        template
            .iter()
            .scan((0, 0), |(i, j), k| {
                if *k == expr::ItemKind::Operator {
                    *i += 1;
                    Some(op[*i - 1].clone())
                } else {
                    *j += 1;
                    Some(num[*j - 1].clone())
                }
            })
            .collect::<Vec<_>>()
    })
    // 全探索して10になる式を抽出
    .filter(|expr| {
        expr::eval(expr)
            .map(|v| v == Ratio::new(10, 1))
            .unwrap_or(false)
    })
    .unique()
    // 出力
    .for_each(|expr| {
        println!("{} = 10", expr::normalize(&expr));
    });
}
