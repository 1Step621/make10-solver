use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub const VALUES: [Operator; std::mem::variant_count::<Operator>()] =
        [Operator::Add, Operator::Sub, Operator::Mul, Operator::Div];

    pub fn apply<T>(&self, a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
            Operator::Div => a / b,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}
